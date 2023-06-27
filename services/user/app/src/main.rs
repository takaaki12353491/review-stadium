mod adapter;
mod config;
mod domain;
mod infra;
mod use_case;

use actix_web::web::Data;
use actix_web::{guard, web, App, HttpServer};
use adapter::{mutation::Mutation, query::Query};
use async_graphql::{EmptySubscription, Schema};
use config::DB_CONFIG;
use infra::user_repository::UserRepositoryImpl;
use opentelemetry::{
    global, runtime::TokioCurrentThread, sdk::propagation::TraceContextPropagator,
};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tracing::*;
use tracing_actix_web::TracingLogger;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use use_case::{mutation_use_case::MutationUseCase, query_use_case::QueryUseCase};

fn init_telemetry() {
    let app_name = "user-app";

    // Start a new Jaeger trace pipeline.
    // Spans are exported in batch - recommended setup for a production application.
    global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name(app_name)
        .install_batch(TokioCurrentThread)
        .expect("Failed to install OpenTelemetry tracer.");

    // Filter based on level - trace, debug, info, warn, error
    // Tunable via `RUST_LOG` env variable
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info"));
    // Create a `tracing` layer using the Jaeger tracer
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    // Create a `tracing` layer to emit spans as structured logs to stdout
    let formatting_layer = BunyanFormattingLayer::new(app_name.into(), std::io::stdout);
    // Combined them all together in a `tracing` subscriber
    let subscriber = Registry::default()
        .with(env_filter)
        .with(telemetry)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to install `tracing` subscriber.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_path("../.env").unwrap();

    init_telemetry();

    let db_url = DB_CONFIG.url();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap();

    let user_repository = Arc::new(UserRepositoryImpl::new(pool));
    let query_use_case = QueryUseCase::new(Arc::clone(&user_repository));
    let mutation_use_case = MutationUseCase::new(Arc::clone(&user_repository));

    let query = Query::new(query_use_case);
    let mutation = Mutation::new(mutation_use_case);
    let schema = Schema::build(query, mutation, EmptySubscription).finish();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .wrap(TracingLogger::default())
            .service(
                web::resource("/")
                    .guard(guard::Post())
                    .to(adapter::user_controller::graphql::<UserRepositoryImpl>),
            )
    })
    .bind(("localhost", 8080))?
    .run()
    .await?;

    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}
