mod constants;
mod graphql;
mod models;
mod repositories;
mod utils;

use crate::graphql::schema;
use crate::graphql::schema::AppSchema;
use crate::repositories::defs::user::UserRepository;
use crate::repositories::impls::postgres::PostgresRepositoryImpl;
use crate::utils::auth::OptionalClaims;
use async_graphql::http;
use async_graphql::http::GraphQLPlaygroundConfig;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use axum::routing::{get, post};
use axum::{Extension, Router, Server, ServiceExt};
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

const DEFAULT_LOG_LEVEL: &str = "info";
const PORT_ENV: &str = "PORT";

async fn graphql_playground() -> impl IntoResponse {
    Html(http::playground_source(GraphQLPlaygroundConfig::new(
        "/api/graphql",
    )))
}

async fn graphql_handler<R: UserRepository>(
    State(repo): State<Arc<R>>,
    claims: OptionalClaims,
    schema: Extension<AppSchema<R>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema
        .execute(req.into_inner().data(claims).data(repo))
        .await
        .into()
}

#[tokio::main]
async fn main() {
    dotenv::from_path("4_backend/.env").unwrap();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(DEFAULT_LOG_LEVEL)),
        )
        .init();

    let graphql_schema = schema::build_schema::<PostgresRepositoryImpl>();

    let repo = Arc::new(PostgresRepositoryImpl::new().await.unwrap());

    let app = Router::new()
        .route("/playground", get(graphql_playground))
        .route("/api/graphql", post(graphql_handler))
        .with_state(repo)
        .layer(Extension(graphql_schema));

    Server::bind(&SocketAddr::from((
        [0, 0, 0, 0],
        env::var(PORT_ENV).unwrap().parse().unwrap(),
    )))
    .serve(app.into_make_service())
    .await
    .unwrap();
}
