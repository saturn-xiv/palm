use std::sync::Arc;

use opensearch::OpenSearch;
use petunia::{jwt::openssl::OpenSsl as Jwt, session::Session};

pub struct Context {
    pub session: Session,
    pub jwt: Arc<Jwt>,
    pub search: Arc<OpenSearch>,
}

impl juniper::Context for Context {}
