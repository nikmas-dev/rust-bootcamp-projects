use std::sync::Arc;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post, put};
use axum::{routing, Json, Router, Server};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

use crate::commands::{role, user};
use crate::models::{RoleDTO, RoleName, RolePermissions, RoleSlug, UserDataDTO, UserId, UserName};
use crate::repositories::defs::role::RoleRepository;
use crate::repositories::defs::user::UserRepository;
use crate::repositories::impls::postgres::PostgresRepositoryImpl;

mod commands;
mod constants;
mod models;
mod repositories;

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
        .nest("/api/users", user_routes)
        .nest("/api/roles", role_routes)
        .with_state(repo);

    Server::bind(&"0.0.0.0:3008".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
