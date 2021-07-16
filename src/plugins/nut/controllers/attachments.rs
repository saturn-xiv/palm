// use std::convert::Infallible;
// use std::ops::Deref;
// use std::result::Result as StdResult;
// use std::sync::Arc;

// use bytes::BufMut;
// use futures::{TryFutureExt, TryStreamExt};
// use hyper::StatusCode;

// use super::super::super::super::{crypto::Aes, jwt::Jwt, orm::postgresql::Pool as DbPool, Result};
// use super::super::models::attachment::Item as Attachment;

// pub async fn post(
//     token: Option<String>,
//     form: warp::multipart::FormData,
//     db: DbPool,
//     jwt: Arc<Jwt>,
//     aes: Arc<Aes>,
// ) -> StdResult<impl warp::Reply, Infallible> {
//     let status = store(token, db, jwt, aes, form)
//         .await
//         .map(|_| StatusCode::OK)
//         .unwrap_or_else(|_| StatusCode::FORBIDDEN);
//     Ok(status)
// }

// async fn store(
//     token: Option<String>,
//     db: DbPool,
//     jwt: Arc<Jwt>,
//     aes: Arc<Aes>,
//     form: warp::multipart::FormData,
// ) -> Result<()> {
//     let files: Vec<(String, Vec<u8>)> = form
//         .and_then(|part| {
//             let name = part.name().to_string();
//             let value = part.stream().try_fold(Vec::new(), |mut vec, data| {
//                 vec.put(data);
//                 async move { Ok(vec) }
//             });
//             value.map_ok(move |vec| (name, vec))
//         })
//         .try_collect()
//         .await?;
//     let user = {
//         let db = db.get()?;
//         let db = db.deref();
//         let jwt = jwt.deref();
//         let ss = Session {
//             token,
//             ..Default::default()
//         };
//         ss.current_user(db, jwt)?
//     };
//     for (name, payload) in files {
//         let db = db.get()?;
//         let aes = aes.deref();
//         Attachment::store(db, aes, user.id, &name, &payload).await?;
//     }
//     Ok(())
// }
