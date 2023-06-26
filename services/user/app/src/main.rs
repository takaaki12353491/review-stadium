mod adapter;
mod config;
mod domain;
mod infra;
mod use_case;

use actix_web::web::Data;
use actix_web::{guard, middleware::Logger, web, App, HttpResponse, HttpServer, Result};
use adapter::{mutation::Mutation, query::Query};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use config::DB_CONFIG;
use infra::user_repository::UserRepositoryImpl;
use log::*;
use sqlx::postgres::PgPoolOptions;
use use_case::{mutation_use_case::MutationUseCase, query_use_case::QueryUseCase};

async fn index_playground() -> Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_path("../.env").unwrap();

    env_logger::init();

    let db_url = DB_CONFIG.url();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap();

    let user_repository = UserRepositoryImpl::new(pool);
    let query_use_case = QueryUseCase::new(user_repository.clone());
    let mutation_use_case = MutationUseCase::new(user_repository.clone());

    let query = Query::new(query_use_case);
    let mutation = Mutation::new(mutation_use_case);
    let schema = Schema::build(query, mutation, EmptySubscription).finish();

    info!("Playground: http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .wrap(Logger::default())
            .service(
                web::resource("/")
                    .guard(guard::Post())
                    .to(adapter::user_controller::graphql::<UserRepositoryImpl>),
            )
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
