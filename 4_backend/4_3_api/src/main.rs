use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::response::IntoResponse;
use axum::routing::{delete, post, put};
use axum::{Router, Server};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use tracing_subscriber::EnvFilter;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::commands::{role, user};
use crate::models::{
    GetUserResultDTO, RoleDTO, RoleName, RolePermissions, RoleSlug, UpdateRoleNameDTO,
    UpdateRolePermissionsDTO, UpdateUserNameDTO, UserDTO, UserDataDTO, UserId, UserName,
};
use crate::repositories::defs::role::RoleRepository;
use crate::repositories::defs::user::UserRepository;
use crate::repositories::impls::postgres::{CombinedRepository, PostgresRepositoryImpl};

mod commands;
mod constants;
mod models;
mod repositories;

const PORT_ENV: &str = "PORT";
const DEFAULT_LOG_LEVEL: &str = "info";

#[derive(OpenApi)]
#[openapi(
    paths(
        user::create_user,
        user::get_all_users,
        user::get_user_by_id,
        user::update_user_name,
        user::add_role_to_user,
        user::remove_role_from_user,
        role::create_role,
        role::get_all_roles,
        role::get_role_by_slug,
        role::update_role_name,
        role::update_role_permissions,
        role::delete_role
    ),
    components(schemas(
        UpdateRoleNameDTO,
        UpdateRolePermissionsDTO,
        UserId,
        GetUserResultDTO,
        UserDataDTO,
        UpdateUserNameDTO,
        UserDTO,
        UserName,
        RoleSlug,
        RoleName,
        RolePermissions,
        RoleDTO
    ))
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    dotenv::from_path("4_backend/4_3_api/.env").unwrap();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(DEFAULT_LOG_LEVEL)),
        )
        .init();

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
        .nest("/api/users", user_routes)
        .nest("/api/roles", role_routes)
        .with_state(repo);

    Server::bind(&SocketAddr::from((
        [0, 0, 0, 0],
        env::var(PORT_ENV).unwrap().parse().unwrap(),
    )))
    .serve(app.into_make_service())
    .await
    .unwrap();
}
