use actix_web::{get, route, web, HttpResponse, Responder};
use actix_web_lab::respond::Html;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

use super::Schema;

pub fn register(config: &mut web::ServiceConfig) {
    config.service(handler).service(playground);
}

#[get("/graphiql")]
async fn playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

#[route("/graphql", method = "GET", method = "POST")]
async fn handler(st: web::Data<Schema>, req: web::Json<GraphQLRequest>) -> impl Responder {
    let res = req.execute(&st, &()).await;
    HttpResponse::Ok().json(res)
}
