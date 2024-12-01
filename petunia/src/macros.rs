#[macro_export]
macro_rules! try_web {
    ($x:expr) => {
        $x.map_err(|e| -> actix_web::Error { actix_web::error::ErrorInternalServerError(e) })
    };
}
