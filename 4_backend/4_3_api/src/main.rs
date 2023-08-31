use std::sync::Arc;

use axum::response::IntoResponse;
use axum::routing::{delete, post, put};
use axum::{Router, Server};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

use crate::commands::{role, user};
use crate::models::{GetUserResultDTO, UserDataDTO};
use crate::repositories::defs::role::RoleRepository;
use crate::repositories::defs::user::UserRepository;
use crate::repositories::impls::postgres::PostgresRepositoryImpl;

mod commands;
mod constants;
mod models;
mod repositories;

#[derive(OpenApi)]
#[openapi(
    paths(
        user::create_user,
        user::get_all_users,
        user::update_user_name,
        user::delete_user,
        user::get_user_by_id,
        user::add_role_to_user,
        user::remove_role_from_user,
        role::create_role,
        role::get_all_roles,
        role::update_role_name,
        role::update_role_permissions,
        role::delete_role,
        role::get_role_by_slug
    ),
    components(schemas(GetUserResultDTO, UserDataDTO))
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    dotenv::from_path("4_backend/4_3_api/.env").unwrap();

    let repo = Arc::new(PostgresRepositoryImpl::new().await.unwrap());

    let user_routes = Router::new()
        .route("/", post(user::create_user).get(user::get_all_users))
        .route("/:id/name", put(user::update_user_name))
        .route("/:id", delete(user::delete_user).get(user::get_user_by_id))
        .route(
            "/:id/roles/:slug",
            post(user::add_role_to_user).delete(user::remove_role_from_user),
        );

    let role_routes = Router::new()
        .route("/", post(role::create_role).get(role::get_all_roles))
        .route("/:slug/name", put(role::update_role_name))
        .route("/:slug/permissions", put(role::update_role_permissions))
        .route(
            "/:slug",
            delete(role::delete_role).get(role::get_role_by_slug),
        );

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        // .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .nest("/api/users", user_routes)
        .nest("/api/roles", role_routes)
        .with_state(repo);

    Server::bind(&"0.0.0.0:3008".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
