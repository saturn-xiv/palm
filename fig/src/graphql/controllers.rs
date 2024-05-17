use actix_web::{get, route, web, HttpResponse, Responder};
use actix_web_lab::respond::Html;
use camelia::{
    controllers::{Gourd, Jasmine, Loquat},
    orm::postgresql::Pool as PostgreSql,
};
use hibiscus::{
    cache::redis::Pool as Redis, handlers::home::Home, queue::rabbitmq::Config as RabbitMq,
    search::Config as OpenSearch, session::Session,
};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

use super::Schema;

pub fn register(config: &mut web::ServiceConfig) {
    config.service(handler).service(playground);
}

#[get("/graphiql")]
async fn playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

type Context = (
    web::Data<Loquat>,
    web::Data<Gourd>,
    web::Data<Jasmine>,
    web::Data<PostgreSql>,
    web::Data<Redis>,
    web::Data<Schema>,
    web::Data<RabbitMq>,
    web::Data<OpenSearch>,
);

#[route("/graphql", method = "GET", method = "POST")]
async fn handler(
    (home, session): (Home, Session),
    (loquat, gourd, jasmine, postgresql, redis, schema, rabbitmq, opensearch): Context,
    request: web::Json<GraphQLRequest>,
) -> impl Responder {
    let context = super::context::Context {
        loquat: loquat.into_inner(),
        gourd: gourd.into_inner(),
        jasmine: jasmine.into_inner(),
        postgresql: postgresql.get_ref().to_owned(),
        redis: redis.get_ref().to_owned(),
        rabbitmq: rabbitmq.into_inner(),
        opensearch: opensearch.into_inner(),
        home: home.to_string(),
        session,
    };
    let response = request.execute(&schema, &context).await;
    HttpResponse::Ok().json(response)
}
