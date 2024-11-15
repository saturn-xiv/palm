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
use hyacinth::graphql as hyacinth_graphql;
use juniper::{graphql_object, FieldResult};
use petunia::{graphql::Succeed, themes::Author as SiteAuthor, Editor};
use wisteria::graphql as wisteria_graphql;

use super::context::Context;

pub struct Mutation;

#[graphql_object(Context = Context)]
impl Mutation {
    async fn install(
        context: &Context,
        lang: String,
        site: daffodil_site::info::Base,
        user: daffodil_user_by_email::SignUp,
    ) -> FieldResult<Succeed> {
        let form = daffodil_site::Install {
            site,
            user: daffodil_user_by_email::SignUp {
                real_name: user.real_name.trim().to_string(),
                email: user.email.trim().to_lowercase(),
                nickname: user.nickname.trim().to_lowercase(),
                password: user.password.clone(),
                timezone: user.timezone,
            },
        };

        let db = context.postgresql.deref();
        let enf = context.enforcer.deref();
        form.execute(db, enf, &lang, &context.session.client_ip)
            .await?;
        Ok(Succeed::default())
    }
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
        lang: String,
        form: daffodil_user_by_email::SignUp,
    ) -> FieldResult<Succeed> {
        let form = daffodil_user_by_email::SignUp {
            real_name: form.real_name.trim().to_string(),
            email: form.email.trim().to_lowercase(),
            nickname: form.nickname.trim().to_lowercase(),
            password: form.password.clone(),
            timezone: form.timezone,
        };
        let db = context.postgresql.deref();
        let queue = context.rabbitmq.deref();
        let jwt = context.jwt.deref();
        form.execute(db, jwt, queue, &lang, &context.session.client_ip)
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
        let form = daffodil_user_by_email::SetProfile {
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
    fn user_sign_out(context: &Context) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        daffodil_user::sign_out(&context.session, db, jwt, &context.session.client_ip)?;
        Ok(Succeed::default())
    }
    fn cancel_my_email_account(
        context: &Context,
        password: String,
        reason: String,
    ) -> FieldResult<Succeed> {
        let form = daffodil_user::email::Cancel { password, reason };
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        form.execute(&context.session, db, jwt, &context.session.client_ip)?;
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
    fn create_leave_word(context: &Context, body: String, editor: Editor) -> FieldResult<Succeed> {
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
    async fn enable_leave_word(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        daffodil_leave_word::enable(&context.session, db, jwt, enf, id).await?;
        Ok(Succeed::default())
    }
    async fn disable_leave_word(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        daffodil_leave_word::disable(&context.session, db, jwt, enf, id).await?;
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
        form: daffodil_site::info::Base,
    ) -> FieldResult<Succeed> {
        let form = daffodil_site::info::Base {
            title: form.title.trim().to_string(),
            subhead: form.subhead.trim().to_string(),
            description: form.description.trim().to_string(),
            copyright: form.copyright.trim().to_string(),
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
    fn create_questionnaire_form(
        context: &Context,
        title: String,
        description: String,
        description_editor: String,
    ) -> FieldResult<Succeed> {
        let form = wisteria_graphql::form::Create {
            title,
            description,
            editor: description_editor.trim().to_lowercase(),
        };
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        form.execute(&context.session, db, jwt)?;
        Ok(Succeed::default())
    }
    fn update_questionnaire_form(
        context: &Context,
        id: i32,
        title: String,
        description: String,
    ) -> FieldResult<Succeed> {
        let form = wisteria_graphql::form::Update { title, description };
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        form.execute(&context.session, db, jwt, id)?;
        Ok(Succeed::default())
    }
    fn lock_questionnaire_form(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        wisteria_graphql::form::lock(&context.session, db, jwt, id)?;
        Ok(Succeed::default())
    }
    fn unlock_questionnaire_form(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        wisteria_graphql::form::unlock(&context.session, db, jwt, id)?;
        Ok(Succeed::default())
    }
    fn enable_questionnaire_form(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        wisteria_graphql::form::enable(&context.session, db, jwt, id)?;
        Ok(Succeed::default())
    }
    fn disable_questionnaire_form(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        wisteria_graphql::form::disable(&context.session, db, jwt, id)?;
        Ok(Succeed::default())
    }
    // ------------------------------------------------------------------------
    fn create_questionnaire_field(
        context: &Context,
        form: i32,
        label: String,
        summary: String,
        sort_order: i32,
    ) -> FieldResult<Succeed> {
        let form_ = wisteria_graphql::field::Create {
            label,
            summary,
            sort_order,
        };
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        form_.execute(&context.session, db, jwt, form)?;
        Ok(Succeed::default())
    }
    fn update_questionnaire_field(
        context: &Context,
        id: i32,
        label: String,
        summary: String,
        sort_order: i32,
    ) -> FieldResult<Succeed> {
        let form = wisteria_graphql::field::Update {
            label,
            summary,
            sort_order,
        };
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        form.execute(&context.session, db, jwt, id)?;
        Ok(Succeed::default())
    }
    fn enable_questionnaire_field(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        wisteria_graphql::field::enable(&context.session, db, jwt, id)?;
        Ok(Succeed::default())
    }
    fn disable_questionnaire_field(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        wisteria_graphql::field::disable(&context.session, db, jwt, id)?;
        Ok(Succeed::default())
    }
    // ------------------------------------------------------------------------
    fn create_questionnaire_pool(context: &Context, form: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        wisteria_graphql::poll::create(&context.session, db, jwt, form)?;
        Ok(Succeed::default())
    }
    fn update_questionnaire_pool(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        wisteria_graphql::poll::update(&context.session, db, jwt, id)?;
        Ok(Succeed::default())
    }
    fn enable_questionnaire_pool(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        wisteria_graphql::poll::enable(&context.session, db, jwt, id)?;
        Ok(Succeed::default())
    }
    fn disable_questionnaire_pool(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        wisteria_graphql::poll::disable(&context.session, db, jwt, id)?;
        Ok(Succeed::default())
    }
    // ------------------------------------------------------------------------
    fn create_bookkeeping_ledger(
        context: &Context,
        label: String,
        memo: String,
    ) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let form = hyacinth_graphql::ledger::Form { label, memo };
        form.create(&context.session, db, jwt)?;
        Ok(Succeed::default())
    }
    async fn update_bookkeeping_ledger(
        context: &Context,
        id: i32,
        label: String,
        memo: String,
    ) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        let form = hyacinth_graphql::ledger::Form { label, memo };
        form.update(&context.session, db, jwt, enf, id).await?;
        Ok(Succeed::default())
    }
    async fn enable_bookkeeping_ledger(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        hyacinth_graphql::ledger::enable(&context.session, db, jwt, enf, id).await?;
        Ok(Succeed::default())
    }
    async fn disable_bookkeeping_ledger(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        hyacinth_graphql::ledger::disable(&context.session, db, jwt, enf, id).await?;
        Ok(Succeed::default())
    }

    async fn create_bookkeeping_category(
        context: &Context,
        ledger: i32,
        parent: Option<i32>,
        label: String,
    ) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        let form = hyacinth_graphql::category::Form { label };
        form.create(&context.session, db, jwt, enf, ledger, parent)
            .await?;
        Ok(Succeed::default())
    }
    async fn update_bookkeeping_category(
        context: &Context,
        id: i32,
        label: String,
    ) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        let form = hyacinth_graphql::category::Form { label };
        form.update(&context.session, db, jwt, enf, id).await?;
        Ok(Succeed::default())
    }
    async fn enable_bookkeeping_category(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        hyacinth_graphql::category::enable(&context.session, db, jwt, enf, id).await?;
        Ok(Succeed::default())
    }
    async fn disable_bookkeeping_category(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        hyacinth_graphql::category::disable(&context.session, db, jwt, enf, id).await?;
        Ok(Succeed::default())
    }

    async fn create_bookkeeping_account(
        context: &Context,
        ledger: i32,
        parent: Option<i32>,
        label: String,
        memo: String,
        currency: i32,
        r#type: hyacinth::models::account::Type,
    ) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        let form = hyacinth_graphql::account::Form { label, memo };
        form.create(
            &context.session,
            db,
            jwt,
            enf,
            (ledger, parent, currency, r#type),
        )
        .await?;
        Ok(Succeed::default())
    }
    async fn update_bookkeeping_account(
        context: &Context,
        id: i32,
        label: String,
        memo: String,
    ) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        let form = hyacinth_graphql::account::Form { label, memo };
        form.update(&context.session, db, jwt, enf, id).await?;
        Ok(Succeed::default())
    }
    async fn enable_bookkeeping_account(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        hyacinth_graphql::account::enable(&context.session, db, jwt, enf, id).await?;
        Ok(Succeed::default())
    }
    async fn disable_bookkeeping_account(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        hyacinth_graphql::account::disable(&context.session, db, jwt, enf, id).await?;
        Ok(Succeed::default())
    }

    async fn create_bookkeeping_merchant(
        context: &Context,
        ledger: i32,
        label: String,
        memo: String,
    ) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        let form = hyacinth_graphql::merchant::Form { label, memo };
        form.create(&context.session, db, jwt, enf, ledger).await?;
        Ok(Succeed::default())
    }
    async fn update_bookkeeping_merchant(
        context: &Context,
        id: i32,
        label: String,
        memo: String,
    ) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        let form = hyacinth_graphql::merchant::Form { label, memo };
        form.update(&context.session, db, jwt, enf, id).await?;
        Ok(Succeed::default())
    }
    async fn enable_bookkeeping_merchant(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        hyacinth_graphql::merchant::enable(&context.session, db, jwt, enf, id).await?;
        Ok(Succeed::default())
    }
    async fn disable_bookkeeping_merchant(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        hyacinth_graphql::merchant::disable(&context.session, db, jwt, enf, id).await?;
        Ok(Succeed::default())
    }

    async fn create_bookkeeping_transaction(
        context: &Context,
        ledger: i32,
        memo: String,
        entries: Vec<hyacinth_graphql::entry::New>,
    ) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        let form = hyacinth_graphql::transaction::Form { memo };
        form.create(&context.session, db, jwt, enf, ledger, &entries)
            .await?;
        Ok(Succeed::default())
    }
    async fn enable_bookkeeping_transaction(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        hyacinth_graphql::transaction::enable(&context.session, db, jwt, enf, id).await?;
        Ok(Succeed::default())
    }
    async fn disable_bookkeeping_transaction(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        hyacinth_graphql::transaction::disable(&context.session, db, jwt, enf, id).await?;
        Ok(Succeed::default())
    }

    async fn create_bookkeeping_entry(
        context: &Context,
        transaction: i32,
        form: hyacinth_graphql::entry::New,
    ) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        form.create(&context.session, db, jwt, enf, transaction)
            .await?;
        Ok(Succeed::default())
    }
    async fn enable_bookkeeping_entry(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        hyacinth_graphql::entry::enable(&context.session, db, jwt, enf, id).await?;
        Ok(Succeed::default())
    }
    async fn disable_bookkeeping_entry(context: &Context, id: i32) -> FieldResult<Succeed> {
        let db = context.postgresql.deref();
        let jwt = context.jwt.deref();
        let enf = context.enforcer.deref();
        hyacinth_graphql::entry::disable(&context.session, db, jwt, enf, id).await?;
        Ok(Succeed::default())
    }
    // ------------------------------------------------------------------------
    // ------------------------------------------------------------------------
}
