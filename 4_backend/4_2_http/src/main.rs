use std::sync::Arc;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::{routing, Json, Router, Server};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

use crate::commands::{role, user};
use crate::models::{Role, RoleName, RolePermissions, RoleSlug, UserData, UserId, UserName};
use crate::repositories::defs::role::RoleRepository;
use crate::repositories::defs::user::UserRepository;
use crate::repositories::impls::postgres::PostgresRepositoryImpl;

mod commands;
mod models;
mod repositories;

#[derive(Deserialize)]
enum Command {
    User(UserCommand),
    Role(RoleCommand),
}

#[derive(Deserialize)]
enum UserCommand {
    Create {
        name: UserName,
    },
    UpdateName {
        id: UserId,
        new_name: UserName,
    },
    Delete {
        id: UserId,
    },
    GetById {
        id: UserId,
    },
    GetAll,
    AddRole {
        user_id: UserId,
        role_slug: RoleSlug,
    },
    RemoveRole {
        user_id: UserId,
        role_slug: RoleSlug,
    },
}

#[derive(Deserialize)]
enum RoleCommand {
    Create {
        slug: RoleSlug,
        name: RoleName,
        permissions: RolePermissions,
    },
    UpdateName {
        slug: RoleSlug,
        new_name: RoleName,
    },
    UpdatePermissions {
        slug: RoleSlug,
        new_permissions: RolePermissions,
    },
    Delete {
        slug: RoleSlug,
    },
    GetBySlug {
        slug: RoleSlug,
    },
    GetAll,
}

#[tokio::main]
async fn main() {
    dotenv::from_path("4_backend/4_2_http/.env").unwrap();

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
            UserCommand::Create { name } => user::create_user(UserData { name }, repo)
                .await
                .into_response(),
            UserCommand::UpdateName { id, new_name } => user::update_user_name(&id, new_name, repo)
                .await
                .into_response(),
            UserCommand::Delete { id } => user::delete_user(&id, repo).await.into_response(),
            UserCommand::GetById { id } => user::get_user_by_id(&id, repo).await.into_response(),
            UserCommand::GetAll => user::get_all_users(repo).await.into_response(),
            UserCommand::AddRole { user_id, role_slug } => {
                user::add_role_to_user(&user_id, &role_slug, repo)
                    .await
                    .into_response()
            }
            UserCommand::RemoveRole { user_id, role_slug } => {
                user::remove_role_from_user(&user_id, &role_slug, repo)
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
                Role {
                    slug,
                    name,
                    permissions,
                },
                repo,
            )
            .await
            .into_response(),
            RoleCommand::UpdateName { slug, new_name } => {
                role::update_role_name(&slug, new_name, repo)
                    .await
                    .into_response()
            }
            RoleCommand::UpdatePermissions {
                slug,
                new_permissions,
            } => role::update_role_permissions(&slug, new_permissions, repo)
                .await
                .into_response(),
            RoleCommand::Delete { slug } => role::delete_role(&slug, repo).await.into_response(),
            RoleCommand::GetBySlug { slug } => {
                role::get_role_by_slug(&slug, repo).await.into_response()
            }
            RoleCommand::GetAll => role::get_all_roles(repo).await.into_response(),
        },
    }
}
