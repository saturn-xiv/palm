use actix_web::{get, route, web, HttpResponse, Responder, Result};
use casbin::Enforcer;
use juniper::http::GraphQLRequest;
use juniper_actix::{graphiql_handler, playground_handler};
use opensearch::OpenSearch;
use petunia::{
    cache::redis::Pool as RedisPool, crypto::Key, jwt::openssl::OpenSsl as Jwt,
    orm::postgresql::Pool as PostgreSqlPool, queue::amqp::RabbitMq, s3::Client as Minio,
    session::Session,
};
use tokio::sync::Mutex;

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

type Context = (
    web::Data<Key>,
    web::Data<PostgreSqlPool>,
    web::Data<RedisPool>,
    web::Data<Jwt>,
    web::Data<Mutex<Enforcer>>,
    web::Data<RabbitMq>,
    web::Data<OpenSearch>,
    web::Data<Minio>,
    web::Data<Schema>,
);

#[route("/graphql", method = "GET", method = "POST")]
async fn handler(
    session: Session,
    (secrets, postgresql, redis, jwt, enforcer, rabbitmq, search, minio, schema): Context,
    request: web::Json<GraphQLRequest>,
) -> impl Responder {
    let context = super::context::Context {
        secrets: secrets.into_inner(),
        jwt: jwt.into_inner(),
        enforcer: enforcer.into_inner(),
        search: search.into_inner(),
        postgresql: postgresql.into_inner(),
        redis: redis.into_inner(),
        rabbitmq: rabbitmq.into_inner(),
        minio: minio.into_inner(),
        session,
    };
    let response = request.execute(&schema, &context).await;
    HttpResponse::Ok().json(response)
}
