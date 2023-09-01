use std::sync::Arc;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::{routing, Json, Router, Server};
use clap::{Parser};

use tracing_subscriber::EnvFilter;

use api::{Command, RoleCommand, UserCommand};

use crate::commands::{role, user};
use crate::models::{
    RoleDTO, UpdateRoleNameDTO, UpdateRolePermissionsDTO, UpdateUserNameDTO, UserDataDTO,
};
use crate::repositories::defs::role::RoleRepository;
use crate::repositories::defs::user::UserRepository;
use crate::repositories::impls::postgres::PostgresRepositoryImpl;

mod commands;
mod constants;
mod models;
mod repositories;

const DEFAULT_LOG_LEVEL: &str = "info";

#[tokio::main]
async fn main() {
    dotenv::from_path("4_backend/4_2_http/.env").unwrap();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(DEFAULT_LOG_LEVEL)),
        )
        .init();

    let repo = Arc::new(PostgresRepositoryImpl::new().await.unwrap());

    let app = Router::new()
        .route("/execute-command", routing::post(execute_command))
        .with_state(repo);

    Server::bind(&"0.0.0.0:3008".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn execute_command<R>(
    State(repo): State<Arc<R>>,
    Json(command): Json<Command>,
) -> impl IntoResponse
where
    R: UserRepository + RoleRepository,
{
    match command {
        Command::User(user_command) => match user_command {
            UserCommand::Create { name } => {
                user::create_user(UserDataDTO { name: name.into() }, repo)
                    .await
                    .into_response()
            }
            UserCommand::UpdateName { id, new_name } => user::update_user_name(
                &id.into(),
                UpdateUserNameDTO {
                    new_name: new_name.into(),
                },
                repo,
            )
            .await
            .into_response(),
            UserCommand::Delete { id } => user::delete_user(&id.into(), repo).await.into_response(),
            UserCommand::GetById { id } => {
                user::get_user_by_id(&id.into(), repo).await.into_response()
            }
            UserCommand::GetAll => user::get_all_users(repo).await.into_response(),
            UserCommand::AddRole { user_id, role_slug } => {
                user::add_role_to_user(&user_id.into(), &role_slug.into(), repo)
                    .await
                    .into_response()
            }
            UserCommand::RemoveRole { user_id, role_slug } => {
                user::remove_role_from_user(&user_id.into(), &role_slug.into(), repo)
                    .await
                    .into_response()
            }
        },
        Command::Role(role_command) => match role_command {
            RoleCommand::Create {
                slug,
                name,
                permissions,
            } => role::create_role(
                RoleDTO {
                    slug: slug.into(),
                    name: name.into(),
                    permissions: permissions.into(),
                },
                repo,
            )
            .await
            .into_response(),
            RoleCommand::UpdateName { slug, new_name } => role::update_role_name(
                &slug.into(),
                UpdateRoleNameDTO {
                    new_name: new_name.into(),
                },
                repo,
            )
            .await
            .into_response(),
            RoleCommand::UpdatePermissions {
                slug,
                new_permissions,
            } => role::update_role_permissions(
                &slug.into(),
                UpdateRolePermissionsDTO {
                    new_permissions: new_permissions.into(),
                },
                repo,
            )
            .await
            .into_response(),
            RoleCommand::Delete { slug } => {
                role::delete_role(&slug.into(), repo).await.into_response()
            }
            RoleCommand::GetBySlug { slug } => role::get_role_by_slug(&slug.into(), repo)
                .await
                .into_response(),
            RoleCommand::GetAll => role::get_all_roles(repo).await.into_response(),
        },
    }
}
