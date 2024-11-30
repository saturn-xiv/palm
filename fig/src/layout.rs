use std::ops::DerefMut;

use casbin::Enforcer;
use daffodil::{
    graphql::user::{CurrentUser, SideBarMenu},
    models::{locale::I18n, user::Item as User},
};
use petunia::{orm::postgresql::Connection as Db, Result};
use tokio::sync::Mutex;

pub trait ExtraSideBarMenus {
    fn append(
        &mut self,
        db: &mut Db,
        enforcer: &Mutex<Enforcer>,
        user: &User,
    ) -> impl std::future::Future<Output = Result<()>>;
}

impl ExtraSideBarMenus for CurrentUser {
    async fn append(&mut self, db: &mut Db, enforcer: &Mutex<Enforcer>, user: &User) -> Result<()> {
        let mut enforcer = enforcer.lock().await;
        let enforcer = enforcer.deref_mut();

        if user.has(enforcer, carnation::graphql::ROLE_MANAGER).is_ok() {
            self.side_bar.push(SideBarMenu {
                label: I18n::t(
                    db,
                    &user.lang,
                    "pages.cms.index.abbreviation",
                    None::<String>,
                ),
                to: "/cms".to_string(),
                icon: Some("cms".to_string()),
                external: false,
                children: None,
            });
        }
        self.side_bar.push(SideBarMenu {
            label: I18n::t(
                db,
                &user.lang,
                "pages.bbs.index.abbreviation",
                None::<String>,
            ),
            to: "/bbs".to_string(),
            icon: Some("bbs".to_string()),
            external: false,
            children: None,
        });

        if user.has(enforcer, hyacinth::graphql::ROLE_MEMBER).is_ok() {
            self.side_bar.push(SideBarMenu {
                label: I18n::t(
                    db,
                    &user.lang,
                    "pages.accounting.index.abbreviation",
                    None::<String>,
                ),
                to: "/accounting".to_string(),
                icon: Some("accounting".to_string()),
                external: false,
                children: None,
            });
        }
        Ok(())
    }
}
