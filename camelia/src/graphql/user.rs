use casbin::{Enforcer, RbacApi};
use chrono::{Duration, NaiveDateTime};
use diesel::Connection as DieselConntection;
use hyper::StatusCode;
use juniper::GraphQLObject;
use palm::{
    cache::redis::ClusterConnection as Cache,
    crypto::Password,
    email::v1 as email_v1,
    jwt::Jwt,
    pagination::{Pager, Pagination},
    queue::rabbitmq::amqp::{Connection as RabbitMq, Protobuf as ProtobufQueue},
    rbac::{v1 as rbac_v1, Permission},
    session::Session,
    to_code, Error, HttpError, Result,
};
use tokio::sync::Mutex;
use validator::Validate;

use super::super::{
    i18n::I18n,
    models::{
        google::user::Dao as GoogleUserDao,
        log::{Dao as LogDao, Level as LogLevel},
        user::{
            session::{Dao as UserSessionDao, ProviderType as UserProviderType},
            Action as UserAction, Dao as UserDao, Item as User,
        },
        wechat::{
            mini_program_user::Dao as WechatMiniProgramUserDao,
            oauth2_user::Dao as WechatOauth2UserDao,
        },
    },
    orm::postgresql::Connection as Db,
    services::CurrentUserAdapter,
    NAME,
};

#[derive(Validate)]
pub struct SignInRequest {
    #[validate(length(min = 1, max = 63))]
    pub user: String,
    #[validate(length(min = 1, max = 31))]
    pub password: String,
    #[validate(range(min = 60, max = 604800))]
    pub ttl: i64,
}

impl SignInRequest {
    pub async fn handle<J: Jwt, P: Password>(
        &self,
        ss: &Session,
        db: &mut Db,
        jwt: &J,
        mac: &P,
        enforcer: &Mutex<Enforcer>,
    ) -> Result<SignInResponse> {
        self.validate()?;
        let user = user_by_nickname_or_email(db, &self.user)?;
        user.auth(mac, &self.password)?;
        db.transaction::<_, Error, _>(move |db| {
            UserDao::sign_in(db, user.id, &ss.client_ip)?;
            LogDao::add::<_, User>(
                db,
                user.id,
                NAME,
                LogLevel::Info,
                &ss.client_ip,
                Some(user.id),
                "sign in success",
            )?;
            Ok(())
        })?;
        let it = new_sign_in_response(
            db,
            enforcer,
            &user,
            jwt,
            Duration::try_seconds(self.ttl).ok_or(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some("bad sign in ttl".to_string()),
            )))?,
            (UserProviderType::Email, user.id),
            &ss.client_ip,
        )
        .await?;
        Ok(it)
    }
}

#[derive(GraphQLObject)]
#[graphql(name = "UserSignInResponse")]
pub struct SignInResponse {
    pub real_name: String,
    pub token: String,
    pub is_administrator: bool,
    pub is_root: bool,
    pub roles: Vec<String>,
    pub permissions: Vec<Permission>,
    pub has_wechat_mini_program: bool,
    pub has_wechat_oauth2: bool,
    pub has_google: bool,
    pub provider_type: String,
}

#[derive(Validate)]
pub struct SignUpRequest {
    #[validate(length(min = 2, max = 31))]
    pub nickname: String,
    #[validate(length(min = 2, max = 31))]
    pub real_name: String,
    #[validate(email, length(min = 6, max = 127))]
    pub email: String,
    #[validate(length(min = 6, max = 31))]
    pub password: String,
    #[validate(length(min = 1, max = 17))]
    pub lang: String,
    #[validate(length(min = 3, max = 31))]
    pub timezone: String,
    #[validate(url, length(min = 6, max = 64))]
    pub home: String,
}

impl SignUpRequest {
    pub async fn handle<J: Jwt, P: Password>(
        &self,
        ss: &Session,
        db: &mut Db,
        jwt: &J,
        mac: &P,
        rabbitmq: &RabbitMq,
    ) -> Result<()> {
        self.validate()?;
        let nickname = to_code!(self.nickname);
        let email = to_code!(self.email);
        let real_name = self.real_name.trim().to_string();

        let user = db.transaction::<_, Error, _>(move |db| {
            UserDao::sign_up(
                db,
                mac,
                &real_name,
                &nickname,
                &email,
                &self.password,
                &self.lang.parse()?,
                &self.timezone.parse()?,
            )?;
            let user = UserDao::by_email(db, &email)?;
            LogDao::add::<_, User>(
                db,
                user.id,
                NAME,
                LogLevel::Info,
                &ss.client_ip,
                Some(user.id),
                "sign up.",
            )?;
            Ok(user)
        })?;

        ByEmail::send_email(db, jwt, rabbitmq, &self.home, &user, &UserAction::Confirm).await?;
        Ok(())
    }
}

#[derive(Validate)]
pub struct ByToken {
    #[validate(length(min = 1, max = 255))]
    pub token: String,
}

impl ByToken {
    pub fn unlock<J: Jwt>(&self, ss: &Session, db: &mut Db, jwt: &J) -> Result<()> {
        self.validate()?;

        let (_, nickname) = jwt.verify(&self.token, NAME, &UserAction::Unlock.to_string())?;
        let user = UserDao::by_nickname(db, &nickname)?;
        if user.locked_at.is_none() {
            return Err(Box::new(HttpError(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(format!("user {} isn't locked!", user.email)),
            )));
        }

        db.transaction::<_, Error, _>(move |db| {
            UserDao::lock(db, user.id, false)?;
            LogDao::add::<String, User>(
                db,
                user.id,
                NAME,
                LogLevel::Info,
                &ss.client_ip,
                Some(user.id),
                "Unlock account.".to_string(),
            )?;
            Ok(())
        })?;

        Ok(())
    }
    pub fn confirm<J: Jwt>(&self, ss: &Session, db: &mut Db, jwt: &J) -> Result<()> {
        self.validate()?;

        let (_, nickname) = jwt.verify(&self.token, NAME, &UserAction::Confirm.to_string())?;
        let user = UserDao::by_nickname(db, &nickname)?;
        if user.confirmed_at.is_some() {
            return Err(Box::new(HttpError(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(format!("user {} already confirmed!", user.email)),
            )));
        }

        db.transaction::<_, Error, _>(move |db| {
            UserDao::confirm(db, user.id)?;
            LogDao::add::<String, User>(
                db,
                user.id,
                NAME,
                LogLevel::Info,
                &ss.client_ip,
                Some(user.id),
                "Confirm account.".to_string(),
            )?;
            Ok(())
        })?;

        Ok(())
    }
}

#[derive(Validate)]
pub struct ResetPassword {
    #[validate(length(min = 6, max = 32))]
    pub password: String,
    #[validate(length(max = 255))]
    pub token: String,
}

impl ResetPassword {
    pub fn handle<J: Jwt, P: Password>(
        &self,
        ss: &Session,
        db: &mut Db,
        jwt: &J,
        hmac: &P,
    ) -> Result<()> {
        self.validate()?;

        let (_, nickname) =
            jwt.verify(&self.token, NAME, &UserAction::ResetPassword.to_string())?;
        let user = UserDao::by_nickname(db, &nickname)?;

        db.transaction::<_, Error, _>(move |db| {
            UserDao::password(db, hmac, user.id, &self.password)?;
            LogDao::add::<String, User>(
                db,
                user.id,
                NAME,
                LogLevel::Info,
                &ss.client_ip,
                Some(user.id),
                "Reset password.".to_string(),
            )?;
            Ok(())
        })?;

        Ok(())
    }
}

#[derive(Validate)]
pub struct RefreshToken {
    #[validate(range(min = 5, max = 604800))]
    pub ttl: i64,
}

impl RefreshToken {
    pub async fn handle<J: Jwt>(
        &self,
        ss: &Session,
        db: &mut Db,
        ch: &mut Cache,
        enf: &Mutex<Enforcer>,
        jwt: &J,
    ) -> Result<SignInResponse> {
        self.validate()?;

        let (user, _, (provider_type, provider_id)) = ss.current_user(db, ch, jwt)?;

        let it = new_sign_in_response(
            db,
            enf,
            &user,
            jwt,
            Duration::try_seconds(self.ttl).ok_or(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some("bad ttl".to_string()),
            )))?,
            (provider_type, provider_id),
            &ss.client_ip,
        )
        .await?;
        Ok(it)
    }
}

#[derive(Validate)]
pub struct ByEmail {
    #[validate(url, length(min = 1, max = 255))]
    pub home: String,
    #[validate(length(min = 1, max = 63))]
    pub user: String,
}

impl ByEmail {
    pub async fn forgot_password<J: Jwt>(
        &self,
        db: &mut Db,
        jwt: &J,
        queue: &RabbitMq,
    ) -> Result<()> {
        self.validate()?;
        let user = user_by_nickname_or_email(db, &self.user)?;
        Self::send_email(
            db,
            jwt,
            queue,
            &self.home,
            &user,
            &UserAction::ResetPassword,
        )
        .await?;
        Ok(())
    }
    pub async fn unlock<J: Jwt>(&self, db: &mut Db, jwt: &J, queue: &RabbitMq) -> Result<()> {
        self.validate()?;

        let user = user_by_nickname_or_email(db, &self.user)?;
        if user.locked_at.is_none() {
            return Err(Box::new(HttpError(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(format!("user {} isn't locked!", user.email)),
            )));
        }
        Self::send_email(db, jwt, queue, &self.home, &user, &UserAction::Unlock).await?;
        Ok(())
    }
    pub async fn confirm<J: Jwt>(&self, db: &mut Db, jwt: &J, queue: &RabbitMq) -> Result<()> {
        self.validate()?;

        let user = user_by_nickname_or_email(db, &self.user)?;
        if user.confirmed_at.is_some() {
            return Err(Box::new(HttpError(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(format!("user {} already confirmed!", user.email)),
            )));
        }

        Self::send_email(db, jwt, queue, &self.home, &user, &UserAction::Confirm).await?;

        Ok(())
    }

    async fn send_email<J: Jwt>(
        db: &mut Db,
        jwt: &J,
        queue: &RabbitMq,
        home: &str,
        user: &User,
        act: &UserAction,
    ) -> Result<()> {
        let act = act.to_string();
        let token = jwt.sign(
            NAME,
            &user.nickname,
            &act,
            Duration::try_hours(1).ok_or(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some("bad ttl".to_string()),
            )))?,
        )?;

        let args = serde_json::json!({
            "real_name": user.real_name,
            "token": token,
            "home": home
        });

        let task = email_v1::SendEmailTask {
            subject: I18n::t(
                db,
                &user.lang,
                format!("auth.mailers.user.{}.subject", act),
                &Some(&args),
            ),
            body: Some(email_v1::send_email_task::Body {
                payload: I18n::t(
                    db,
                    &user.lang,
                    format!("auth.mailers.user.{}.body", act),
                    &Some(&args),
                ),
                html: true,
            }),
            to: Some(email_v1::send_email_task::Address {
                name: user.real_name.clone(),
                email: user.email.clone(),
            }),
            cc: Vec::new(),
            bcc: Vec::new(),
            attachments: Vec::new(),
        };

        ProtobufQueue::produce(queue, &task).await?;
        Ok(())
    }
}

pub fn sign_out<J: Jwt>(ss: &Session, db: &mut Db, ch: &mut Cache, jwt: &J) -> Result<()> {
    let (user, _, _) = ss.current_user(db, ch, jwt)?;
    LogDao::add::<_, User>(
        db,
        user.id,
        NAME,
        LogLevel::Info,
        &ss.client_ip,
        Some(user.id),
        "Sign out.",
    )?;
    Ok(())
}

#[derive(Validate)]
pub struct UpdateProfile {
    #[validate(length(min = 2, max = 31))]
    pub real_name: String,
    #[validate(url, length(max = 127))]
    pub avatar: String,
    #[validate(length(min = 2, max = 15))]
    pub lang: String,
    #[validate(length(min = 2, max = 32))]
    pub timezone: String,
}

impl UpdateProfile {
    pub fn handle<J: Jwt>(&self, ss: &Session, db: &mut Db, ch: &mut Cache, jwt: &J) -> Result<()> {
        self.validate()?;

        let (user, _, _) = ss.current_user(db, ch, jwt)?;

        db.transaction::<_, Error, _>(move |db| {
            UserDao::set_profile(
                db,
                user.id,
                &self.real_name,
                &self.avatar,
                &self.lang.parse()?,
                &self.timezone.parse()?,
            )?;
            LogDao::add::<String, User>(
                db,
                user.id,
                NAME,
                LogLevel::Info,
                &ss.client_ip,
                Some(user.id),
                "Update profile.".to_string(),
            )?;
            Ok(())
        })?;
        Ok(())
    }
}

#[derive(Validate)]
pub struct ChangePassword {
    #[validate(length(min = 1))]
    pub current_password: String,
    #[validate(length(min = 6, max = 32))]
    pub new_password: String,
}

impl ChangePassword {
    pub fn handle<J: Jwt, P: Password>(
        &self,
        ss: &Session,
        db: &mut Db,
        ch: &mut Cache,
        jwt: &J,
        hmac: &P,
    ) -> Result<()> {
        self.validate()?;

        let (user, _, _) = ss.current_user(db, ch, jwt)?;

        user.auth(hmac, &self.current_password)?;
        db.transaction::<_, Error, _>(move |db| {
            UserDao::password(db, hmac, user.id, &self.new_password)?;
            LogDao::add::<String, User>(
                db,
                user.id,
                NAME,
                LogLevel::Info,
                &ss.client_ip,
                Some(user.id),
                "Change password.".to_string(),
            )?;
            Ok(())
        })?;

        Ok(())
    }
}

#[derive(GraphQLObject)]
#[graphql(name = "UserIndexResponseItem")]
pub struct IndexResponseItem {
    pub id: i32,
    pub real_name: String,
    pub nickname: String,
    pub email: String,
    pub avatar: String,
    pub lang: String,
    pub timezone: String,
    pub status: String,
    pub sign_in_count: i32,
    pub current_sign_in_at: Option<NaiveDateTime>,
    pub current_sign_in_ip: Option<String>,
    pub last_sign_in_at: Option<NaiveDateTime>,
    pub last_sign_in_ip: Option<String>,
    pub confirmed_at: Option<NaiveDateTime>,
    pub locked_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

impl From<User> for IndexResponseItem {
    fn from(x: User) -> Self {
        Self {
            id: x.id,
            real_name: x.real_name.clone(),
            nickname: x.nickname.clone(),
            email: x.email.clone(),
            avatar: x.avatar.clone(),
            lang: x.lang.clone(),
            timezone: x.timezone.clone(),
            status: x.status.clone(),
            sign_in_count: x.sign_in_count,
            current_sign_in_at: x.current_sign_in_at,
            current_sign_in_ip: x.current_sign_in_ip.clone(),
            last_sign_in_at: x.last_sign_in_at,
            last_sign_in_ip: x.last_sign_in_ip.clone(),
            confirmed_at: x.confirmed_at,
            locked_at: x.locked_at,
            deleted_at: x.deleted_at,
            updated_at: x.updated_at,
        }
    }
}

impl IndexResponseItem {
    pub async fn new<J: Jwt>(
        ss: &Session,
        db: &mut Db,
        ch: &mut Cache,
        enf: &Mutex<Enforcer>,
        jwt: &J,
        user: i32,
    ) -> Result<Self> {
        {
            let (user, _, _) = ss.current_user(db, ch, jwt)?;
            user.is_administrator(enf).await?;
        }

        let it = UserDao::by_id(db, user)?;
        Ok(it.into())
    }
}

#[derive(GraphQLObject)]
#[graphql(name = "UserIndexResponse")]
pub struct IndexResponse {
    pub items: Vec<IndexResponseItem>,
    pub pagination: Pagination,
}

impl IndexResponse {
    pub async fn new<J: Jwt>(
        ss: &Session,
        db: &mut Db,
        ch: &mut Cache,
        enf: &Mutex<Enforcer>,
        jwt: &J,
        pager: &Pager,
    ) -> Result<Self> {
        {
            let (user, _, _) = ss.current_user(db, ch, jwt)?;
            user.is_administrator(enf).await?;
        }

        let total = UserDao::count(db)?;
        let items = UserDao::all(db, pager.offset(total), pager.size())?
            .into_iter()
            .map(|x| x.into())
            .collect();
        Ok(IndexResponse {
            items,
            pagination: Pagination::new(pager, total),
        })
    }
}

pub async fn enable<J: Jwt>(
    ss: &Session,
    db: &mut Db,
    ch: &mut Cache,
    enf: &Mutex<Enforcer>,
    jwt: &J,
    id: i32,
) -> Result<()> {
    let (user, _, _) = ss.current_user(db, ch, jwt)?;
    user.is_administrator(enf).await?;

    let it = UserDao::by_id(db, id)?;
    if it.is_root(enf).await.is_ok() {
        return Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)));
    }
    db.transaction::<_, Error, _>(move |db| {
        UserDao::enable(db, id, true)?;
        LogDao::add::<_, User>(
            db,
            it.id,
            NAME,
            LogLevel::Info,
            &ss.client_ip,
            Some(it.id),
            &format!("enable by {}", user),
        )?;
        Ok(())
    })?;
    Ok(())
}

pub async fn disable<J: Jwt>(
    ss: &Session,
    db: &mut Db,
    ch: &mut Cache,
    enf: &Mutex<Enforcer>,
    jwt: &J,
    id: i32,
) -> Result<()> {
    let (user, _, _) = ss.current_user(db, ch, jwt)?;
    user.is_administrator(enf).await?;

    let it = UserDao::by_id(db, id)?;
    if it.is_root(enf).await.is_ok() {
        return Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)));
    }
    db.transaction::<_, Error, _>(move |db| {
        UserDao::enable(db, id, false)?;
        LogDao::add::<_, User>(
            db,
            it.id,
            NAME,
            LogLevel::Info,
            &ss.client_ip,
            Some(it.id),
            &format!("disable by {}", user),
        )?;
        Ok(())
    })?;
    Ok(())
}

pub async fn lock<J: Jwt>(
    ss: &Session,
    db: &mut Db,
    ch: &mut Cache,
    enf: &Mutex<Enforcer>,
    jwt: &J,
    id: i32,
) -> Result<()> {
    let (user, _, _) = ss.current_user(db, ch, jwt)?;
    user.is_administrator(enf).await?;

    let it = UserDao::by_id(db, id)?;
    if it.is_root(enf).await.is_ok() {
        return Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)));
    }
    db.transaction::<_, Error, _>(move |db| {
        UserDao::lock(db, id, true)?;
        LogDao::add::<_, User>(
            db,
            it.id,
            NAME,
            LogLevel::Info,
            &ss.client_ip,
            Some(it.id),
            &format!("lock by {}", user),
        )?;
        Ok(())
    })?;
    Ok(())
}

pub async fn unlock<J: Jwt>(
    ss: &Session,
    db: &mut Db,
    ch: &mut Cache,
    enf: &Mutex<Enforcer>,
    jwt: &J,
    id: i32,
) -> Result<()> {
    let (user, _, _) = ss.current_user(db, ch, jwt)?;
    user.is_administrator(enf).await?;

    let it = UserDao::by_id(db, id)?;
    if it.is_root(enf).await.is_ok() {
        return Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)));
    }
    db.transaction::<_, Error, _>(move |db| {
        UserDao::lock(db, id, false)?;
        LogDao::add::<_, User>(
            db,
            it.id,
            NAME,
            LogLevel::Info,
            &ss.client_ip,
            Some(it.id),
            &format!("unlock by {}", user),
        )?;
        Ok(())
    })?;
    Ok(())
}

pub async fn confirm<J: Jwt>(
    ss: &Session,
    db: &mut Db,
    ch: &mut Cache,
    enf: &Mutex<Enforcer>,
    jwt: &J,
    id: i32,
) -> Result<()> {
    let (user, _, _) = ss.current_user(db, ch, jwt)?;
    user.is_administrator(enf).await?;

    let it = UserDao::by_id(db, id)?;
    if it.is_root(enf).await.is_ok() {
        return Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)));
    }
    db.transaction::<_, Error, _>(move |db| {
        UserDao::confirm(db, id)?;
        LogDao::add::<_, User>(
            db,
            it.id,
            NAME,
            LogLevel::Info,
            &ss.client_ip,
            Some(it.id),
            &format!("confirmed by {}", user),
        )?;
        Ok(())
    })?;
    Ok(())
}

#[derive(Validate)]
pub struct SetPassword {
    #[validate(range(min = 1))]
    pub user: i32,
    #[validate(length(min = 6, max = 32))]
    pub password: String,
}
impl SetPassword {
    pub async fn handle<J: Jwt, P: Password>(
        &self,
        ss: &Session,
        db: &mut Db,
        ch: &mut Cache,
        enf: &Mutex<Enforcer>,
        jwt: &J,
        hmac: &P,
    ) -> Result<()> {
        let (user, _, _) = ss.current_user(db, ch, jwt)?;

        user.is_administrator(enf).await?;

        let it = UserDao::by_id(db, self.user)?;
        if it.is_root(enf).await.is_ok() {
            return Err(Box::new(HttpError(StatusCode::FORBIDDEN, None)));
        }

        db.transaction::<_, Error, _>(move |db| {
            UserDao::password(db, hmac, self.user, &self.password)?;
            LogDao::add::<_, User>(
                db,
                it.id,
                NAME,
                LogLevel::Info,
                &ss.client_ip,
                Some(it.id),
                &format!("reset password by {}", user),
            )?;
            Ok(())
        })?;
        Ok(())
    }
}
// ----------------------------------------------------------------------------

fn user_by_nickname_or_email(db: &mut Db, user: &str) -> Result<User> {
    if let Ok(it) = UserDao::by_nickname(db, user) {
        return Ok(it);
    }
    UserDao::by_email(db, user)
}

async fn new_sign_in_response<J: Jwt>(
    db: &mut Db,
    enforcer: &Mutex<Enforcer>,
    user: &User,
    jwt: &J,
    ttl: Duration,
    provider: (UserProviderType, i32),
    ip: &str,
) -> Result<SignInResponse> {
    let is_administrator = user.is_administrator(enforcer).await.is_ok();
    let is_root = user.is_root(enforcer).await.is_ok();
    let has_wechat_mini_program = WechatMiniProgramUserDao::count_by_user(db, user.id)? > 0;
    let has_wechat_oauth2 = WechatOauth2UserDao::count_by_user(db, user.id)? > 0;
    let has_google = GoogleUserDao::count_by_user(db, user.id)? > 0;

    let token = {
        let uid = UserSessionDao::create(db, user.id, &provider.0, provider.1, ip, ttl)?;
        jwt.sign(NAME, &uid, &UserAction::SignIn.to_string(), ttl)?
    };

    let user_s = user.to_subject();
    let mut enforcer = enforcer.lock().await;

    let mut roles = Vec::new();
    {
        let items = enforcer.get_implicit_roles_for_user(&user_s, None);
        for it in items.iter() {
            if let Ok(ref it) = it.parse::<rbac_v1::Role>() {
                if let Some(rbac_v1::role::By::Member(ref it)) = it.by {
                    roles.push(it.clone());
                }
            }
        }
    }
    let permissions = {
        let mut items = Vec::new();
        for it in
            rbac_v1::Permission::all(&enforcer.get_implicit_permissions_for_user(&user_s, None))
                .iter()
        {
            if let Some(it) = Permission::new(it) {
                items.push(it);
            }
        }
        items
    };
    Ok(SignInResponse {
        real_name: user.real_name.clone(),
        token,
        roles,
        permissions,
        has_google,
        has_wechat_mini_program,
        has_wechat_oauth2,
        provider_type: provider.0.to_string(),
        is_administrator,
        is_root,
    })
}
