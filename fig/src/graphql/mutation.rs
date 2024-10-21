use std::ops::Deref;

use carnation::graphql::page as cms_page;
use daffodil::graphql::{
    attachment as daffodil_attachment, category as daffodil_category,
    leave_word as daffodil_leave_word, locale as daffodil_locale, menu as daffodil_menu,
    session as daffodil_session, site as daffodil_site, tag as daffodil_tag,
    user::{
        self as daffodil_user, email as daffodil_user_by_email,
        SignInResponse as UserSignInResponse,
    },
};
use juniper::{graphql_object, FieldResult};
use petunia::{graphql::Succeed, themes::Author as SiteAuthor};

use super::context::Context;

pub struct Mutation;

#[graphql_object(Context = Context)]
impl Mutation {
    async fn user_sign_in_by_email(
        context: &Context,
        user: String,
        password: String,
    ) -> FieldResult<UserSignInResponse> {
        let form = daffodil_user_by_email::SignIn {
            user: user.trim().to_lowercase(),
            password,
        };
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();

        let it = form
            .execute(db, jwt, enf, &context.session.client_ip)
            .await?;
        Ok(it)
    }
    async fn user_sign_up_by_email(
        context: &Context,
        real_name: String,
        nickname: String,
        email: String,
        password: String,
        timezone: String,
    ) -> FieldResult<Succeed> {
        let form = daffodil_user_by_email::SignUp {
            real_name: real_name.trim().to_string(),
            email: email.trim().to_lowercase(),
            nickname: nickname.trim().to_lowercase(),
            password,
            timezone,
        };
        let db = context.postgresql.deref();
        let queue = context.rabbitmq.deref();
        let jwt = context.jwt.deref();
        form.execute(
            db,
            jwt,
            queue,
            &context.session.lang,
            &context.session.client_ip,
        )
        .await?;
        Ok(Succeed::default())
    }

    fn unlock_email_user_by_token(context: &Context, token: String) -> FieldResult<Succeed> {
        let form = daffodil_user_by_email::Token { token };
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        form.unlock(db, jwt, &context.session.client_ip)?;
        Ok(Succeed::default())
    }
    fn confirm_email_user_by_token(context: &Context, token: String) -> FieldResult<Succeed> {
        let form = daffodil_user_by_email::Token { token };
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        form.confirm(db, jwt, &context.session.client_ip)?;
        Ok(Succeed::default())
    }
    fn reset_email_user_password_by_token(
        context: &Context,
        token: String,
        password: String,
    ) -> FieldResult<Succeed> {
        let form = daffodil_user_by_email::ResetPassword { token, password };
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        form.execute(db, jwt, &context.session.client_ip)?;
        Ok(Succeed::default())
    }
    async fn confirm_email_user(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        daffodil_user_by_email::confirm(
            &context.session,
            db,
            jwt,
            enf,
            id,
            &context.session.client_ip,
        )
        .await?;
        Ok(Succeed::default())
    }
    async fn enable_email_user(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        daffodil_user_by_email::enable(
            &context.session,
            db,
            jwt,
            enf,
            id,
            &context.session.client_ip,
        )
        .await?;
        Ok(Succeed::default())
    }
    async fn disable_email_user(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        daffodil_user_by_email::disable(
            &context.session,
            db,
            jwt,
            enf,
            id,
            &context.session.client_ip,
        )
        .await?;
        Ok(Succeed::default())
    }

    async fn set_email_user_password(
        context: &Context,
        id: i32,
        password: String,
    ) -> FieldResult<Succeed> {
        let form = daffodil_user_by_email::SetPassword { id, password };
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        form.execute(&context.session, db, jwt, enf, &context.session.client_ip)
            .await?;
        Ok(Succeed::default())
    }
    fn set_email_user_profile(
        context: &Context,
        real_name: String,
        lang: String,
        timezone: String,
    ) -> FieldResult<Succeed> {
        let form = daffodil_user_by_email::Profile {
            real_name: real_name.trim().to_string(),
            lang,
            timezone,
        };
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        form.execute(&context.session, db, jwt, &context.session.client_ip)?;
        Ok(Succeed::default())
    }
    fn change_email_user_password(
        context: &Context,
        current_password: String,
        new_password: String,
    ) -> FieldResult<Succeed> {
        let form = daffodil_user_by_email::ChangePassword {
            current_password,
            new_password,
        };
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        form.execute(&context.session, db, jwt, &context.session.client_ip)?;
        Ok(Succeed::default())
    }
    // ------------------------------------------------------------------------

    async fn enable_user(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        daffodil_user::enable(
            &context.session,
            db,
            jwt,
            enf,
            id,
            &context.session.client_ip,
        )
        .await?;
        Ok(Succeed::default())
    }
    async fn disable_user(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        daffodil_user::disable(
            &context.session,
            db,
            jwt,
            enf,
            id,
            &context.session.client_ip,
        )
        .await?;
        Ok(Succeed::default())
    }
    async fn lock_user(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        daffodil_user::lock(
            &context.session,
            db,
            jwt,
            enf,
            id,
            &context.session.client_ip,
        )
        .await?;
        Ok(Succeed::default())
    }
    async fn unlock_user(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        daffodil_user::unlock(
            &context.session,
            db,
            jwt,
            enf,
            id,
            &context.session.client_ip,
        )
        .await?;
        Ok(Succeed::default())
    }
    // ------------------------------------------------------------------------
    async fn set_locale(
        context: &Context,
        lang: String,
        code: String,
        message: String,
    ) -> FieldResult<Succeed> {
        let form = daffodil_locale::Set {
            lang: lang.trim().to_string(),
            code: code.trim().to_lowercase(),
            message: message.trim().to_string(),
        };
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        form.execute(&context.session, db, jwt, enf).await?;
        Ok(Succeed::default())
    }
    async fn destroy_locale(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        daffodil_locale::destroy(&context.session, db, jwt, enf, id).await?;
        Ok(Succeed::default())
    }
    // ------------------------------------------------------------------------
    fn set_attachment_title(context: &Context, id: i32, title: String) -> FieldResult<Succeed> {
        let form = daffodil_attachment::SetTitle {
            title: title.trim().to_string(),
        };
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        form.execute(&context.session, db, jwt, id)?;
        Ok(Succeed::default())
    }
    fn set_attachment_uploaded_at(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        daffodil_attachment::set_uploaded_at(&context.session, db, jwt, id)?;
        Ok(Succeed::default())
    }
    fn destroy_attachment(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        daffodil_attachment::destroy(&context.session, db, jwt, id)?;
        Ok(Succeed::default())
    }
    // ------------------------------------------------------------------------
    fn create_leave_word(context: &Context, body: String, editor: String) -> FieldResult<Succeed> {
        let form = daffodil_leave_word::Create { body, editor };
        let db = context.postgresql.deref();
        form.execute(&context.session, db)?;
        Ok(Succeed::default())
    }
    async fn close_leave_word(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        daffodil_leave_word::close(&context.session, db, jwt, enf, id).await?;
        Ok(Succeed::default())
    }
    async fn destroy_leave_word(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        daffodil_leave_word::destroy(&context.session, db, jwt, enf, id).await?;
        Ok(Succeed::default())
    }
    // ------------------------------------------------------------------------
    async fn disable_session(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        daffodil_session::disable(&context.session, db, jwt, enf, id).await?;
        Ok(Succeed::default())
    }
    async fn enable_session(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        daffodil_session::enable(&context.session, db, jwt, enf, id).await?;
        Ok(Succeed::default())
    }
    // ------------------------------------------------------------------------
    async fn create_tag(context: &Context, code: String) -> FieldResult<Succeed> {
        let form = daffodil_tag::Form {
            code: code.trim().to_lowercase(),
        };
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        form.create(&context.session, db, jwt, enf).await?;
        Ok(Succeed::default())
    }
    async fn update_tag(context: &Context, id: i32, code: String) -> FieldResult<Succeed> {
        let form = daffodil_tag::Form {
            code: code.trim().to_lowercase(),
        };
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        form.update(&context.session, db, jwt, enf, id).await?;
        Ok(Succeed::default())
    }
    async fn destroy_tag(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        daffodil_tag::destroy(&context.session, db, jwt, enf, id).await?;
        Ok(Succeed::default())
    }
    // ------------------------------------------------------------------------
    async fn create_category(context: &Context, parent: i32, code: String) -> FieldResult<Succeed> {
        let form = daffodil_category::Form {
            code: code.trim().to_lowercase(),
        };
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        form.create(&context.session, db, jwt, enf, parent).await?;
        Ok(Succeed::default())
    }
    async fn append_category(context: &Context, near: i32, code: String) -> FieldResult<Succeed> {
        let form = daffodil_category::Form {
            code: code.trim().to_lowercase(),
        };
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        form.append(&context.session, db, jwt, enf, near).await?;
        Ok(Succeed::default())
    }
    async fn update_category(context: &Context, id: i32, code: String) -> FieldResult<Succeed> {
        let form = daffodil_category::Form {
            code: code.trim().to_lowercase(),
        };
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        form.update(&context.session, db, jwt, enf, id).await?;
        Ok(Succeed::default())
    }
    async fn destroy_category(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        daffodil_category::destroy(&context.session, db, jwt, enf, id).await?;
        Ok(Succeed::default())
    }
    // ------------------------------------------------------------------------
    async fn append_menu(context: &Context, form: daffodil_menu::Append) -> FieldResult<Succeed> {
        let form = {
            let mut it = form.clone();
            it.location = form.location.trim().to_lowercase();
            it
        };
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        form.execute(&context.session, db, jwt, enf).await?;
        Ok(Succeed::default())
    }
    async fn create_menu(
        context: &Context,
        parent: i32,
        form: daffodil_menu::Form,
    ) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        form.create(&context.session, db, jwt, enf, parent).await?;
        Ok(Succeed::default())
    }
    async fn update_menu(
        context: &Context,
        id: i32,
        form: daffodil_menu::Form,
    ) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        form.update(&context.session, db, jwt, enf, id).await?;
        Ok(Succeed::default())
    }
    async fn destroy_menu(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        daffodil_menu::destroy(&context.session, db, jwt, enf, id).await?;
        Ok(Succeed::default())
    }
    // ------------------------------------------------------------------------
    async fn set_site_base_info(
        context: &Context,
        lang: String,
        title: String,
        subhead: String,
        description: String,
        copyright: String,
    ) -> FieldResult<Succeed> {
        let form = daffodil_site::info::Base {
            title: title.trim().to_string(),
            subhead: subhead.trim().to_string(),
            description: description.trim().to_string(),
            copyright: copyright.trim().to_string(),
        };
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        form.save(&context.session, db, jwt, enf, &lang).await?;
        Ok(Succeed::default())
    }
    async fn set_site_keywords(context: &Context, items: Vec<String>) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        let secrets = context.secrets.deref();
        daffodil_site::info::Keywords::save(
            &context.session,
            db,
            secrets.clone(),
            jwt,
            enf,
            &items,
        )
        .await?;
        Ok(Succeed::default())
    }
    async fn set_site_author(
        context: &Context,
        lang: String,
        name: String,
        email: String,
    ) -> FieldResult<Succeed> {
        let form = SiteAuthor {
            name: name.trim().to_string(),
            email: email.trim().to_lowercase(),
        };
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        let secrets = context.secrets.deref();
        daffodil_site::info::Author::save(
            &context.session,
            db,
            secrets.clone(),
            jwt,
            enf,
            &lang,
            &form,
        )
        .await?;
        Ok(Succeed::default())
    }
    // ------------------------------------------------------------------------
    fn create_cms_page(context: &Context, form: cms_page::Create) -> FieldResult<Succeed> {
        let form = {
            let mut it = form.clone();
            it.template = form.template.trim().to_lowercase();
            it.slug = form.slug.trim().to_lowercase();
            it
        };
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        form.execute(&context.session, db, jwt)?;
        Ok(Succeed::default())
    }
    fn update_cms_page(
        context: &Context,
        id: i32,
        slug: String,
        body: String,
    ) -> FieldResult<Succeed> {
        let form = cms_page::Update {
            slug: slug.trim().to_lowercase(),
            body,
        };
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        form.execute(&context.session, db, jwt, id)?;
        Ok(Succeed::default())
    }
    fn set_cms_page_template(context: &Context, id: i32, template: String) -> FieldResult<Succeed> {
        let form = cms_page::SetTemplate {
            template: template.trim().to_lowercase(),
        };
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        form.execute(&context.session, db, jwt, id)?;
        Ok(Succeed::default())
    }
    // ------------------------------------------------------------------------
    // ------------------------------------------------------------------------
}
