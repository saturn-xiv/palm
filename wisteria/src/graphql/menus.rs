use lavender::graphql::ROLE as LavenderOperator;
use portal::{
    Jwt, Result,
    cache::redis::StandaloneConnection as Cache,
    graphql::{Menu, Session},
    orm::postgresql::Connection as Db,
    rbac::Rbac,
};

pub async fn dashboard<R: Rbac, J: Jwt>(
    ss: &Session,
    db: &mut Db,
    cache: &mut Cache,
    rbac: &R,
    jwt: &J,
) -> Result<Vec<Menu>> {
    let current_user = ss.current_user(db, cache, jwt).await?;
    let is_administrator = rbac.is_administrator(current_user.id()).await.is_ok();
    let mut items = vec![Menu {
        code: "personal".to_string(),
        children: Some(vec![]),
        ..Default::default()
    }];

    if is_administrator {
        items.push(Menu {
            code: "site".to_string(),
            children: Some(vec![]),
            ..Default::default()
        });
    }
    if rbac
        .has_role(current_user.id(), LavenderOperator)
        .await
        .is_ok()
    {
        items.push(Menu {
            code: "lavender".to_string(),
            children: Some(vec![]),
            ..Default::default()
        });
    }

    Ok(items)
}
