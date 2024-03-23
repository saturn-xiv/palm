use actix_web::{get, route, web, HttpResponse, Responder};
use actix_web_lab::respond::Html;
use camelia::orm::postgresql::Pool as PostgreSql;
use casbin::Enforcer;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use palm::{
    cache::redis::Pool as Redis,
    crypto::{aes::Aes, hmac::Hmac},
    handlers::home::Home,
    jwt::openssl::Jwt,
    minio::Client as Minio,
    queue::rabbitmq::amqp::Connection as RabbitMq,
    search::Pool as OpenSearch,
    session::Session,
};
use tokio::sync::Mutex;

use super::Schema;

pub fn register(config: &mut web::ServiceConfig) {
    config.service(handler).service(playground);
}

#[get("/graphiql")]
async fn playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

type Context = (
    web::Data<Jwt>,
    web::Data<Aes>,
    web::Data<Hmac>,
    web::Data<PostgreSql>,
    web::Data<Redis>,
    web::Data<Schema>,
    web::Data<RabbitMq>,
    web::Data<Minio>,
    web::Data<OpenSearch>,
    web::Data<Mutex<Enforcer>>,
);

#[route("/graphql", method = "GET", method = "POST")]
async fn handler(
    (home, session): (Home, Session),
    (jwt, aes, hmac, postgresql, redis, schema, rabbitmq, minio, opensearch, enforcer): Context,
    request: web::Json<GraphQLRequest>,
) -> impl Responder {
    let context = super::context::Context {
        jwt: jwt.into_inner(),
        hmac: hmac.into_inner(),
        aes: aes.into_inner(),
        postgresql: postgresql.get_ref().to_owned(),
        redis: redis.get_ref().to_owned(),
        minio: minio.into_inner(),
        rabbitmq: rabbitmq.into_inner(),
        opensearch: opensearch.into_inner(),
        enforcer: enforcer.into_inner(),

        home: home.to_string(),
        session,
    };
    let response = request.execute(&schema, &context).await;
    HttpResponse::Ok().json(response)
}
