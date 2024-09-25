use actix_web::{get, route, web, HttpResponse, Responder, Result};
use juniper::http::GraphQLRequest;
use juniper_actix::{graphiql_handler, playground_handler};
use opensearch::OpenSearch;
use petunia::{jwt::openssl::OpenSsl as Jwt, session::Session};

use super::Schema;

pub fn register(config: &mut web::ServiceConfig) {
    config
        .service(handler)
        .service(playground)
        .service(graphiql);
}

#[get("/playground")]
async fn playground() -> Result<HttpResponse> {
    playground_handler("/graphql", None).await
}
#[get("/graphiql")]
async fn graphiql() -> Result<HttpResponse> {
    graphiql_handler("/graphql", None).await
}

type Context = (web::Data<Jwt>, web::Data<OpenSearch>, web::Data<Schema>);

#[route("/graphql", method = "GET", method = "POST")]
async fn handler(
    session: Session,
    (jwt, search, schema): Context,
    request: web::Json<GraphQLRequest>,
) -> impl Responder {
    let context = super::context::Context {
        jwt: jwt.into_inner(),
        search: search.into_inner(),
        session,
    };
    let response = request.execute(&schema, &context).await;
    HttpResponse::Ok().json(response)
}
