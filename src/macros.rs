// #[macro_export]
// macro_rules! __try {
//     ($v:expr) => {{
//         $v.map_err(|it| {
//             let it = Error::from(it);
//             HttpError(StatusCode::INTERNAL_SERVER_ERROR, Some(it.to_string()))
//         })?
//     }};
// }

// #[macro_export]
// macro_rules! __html {
//     ($v:expr) => {{
//         let (body, status) = match $v {
//             Ok(v) => (v, hyper::StatusCode::OK),
//             Err(e) => {
//                 log::error!("{:#?}", e);
//                 (e.to_string(), hyper::StatusCode::INTERNAL_SERVER_ERROR)
//             }
//         };
//         Ok(warp::reply::with_status(warp::reply::html(body), status))
//     }};
// }

// #[macro_export]
// macro_rules! __plain_text {
//     ($s:expr, $v:expr) => {
//         match $v {
//             Ok(v) => gotham::helpers::http::response::create_response(
//                 $s,
//                 hyper::StatusCode::OK,
//                 mime::TEXT_PLAIN,
//                 v.into_bytes(),
//             ),
//             Err(e) => gotham::helpers::http::response::create_response(
//                 $s,
//                 hyper::StatusCode::INTERNAL_SERVER_ERROR,
//                 mime::TEXT_PLAIN,
//                 format!("{:#?}", e).into_bytes(),
//             ),
//         }
//     };
// }
