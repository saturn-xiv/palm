use super::{
    Result, cache::redis::StandaloneConnection as Cache, graphql::Session,
    orm::postgresql::Connection as Db,
};

pub async fn home(_ss: &Session, _db: &mut Db, _cache: &mut Cache) -> Result<String> {
    // TODO
    Ok("<html><h1>Home</h1></html>".to_string())
}
