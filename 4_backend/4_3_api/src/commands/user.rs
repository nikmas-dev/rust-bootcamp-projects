use crate::models::{
    GetUserResultDTO, RoleSlug, UpdateUserNameDTO, UserDTO, UserDataDTO, UserId, UserName,
};
use crate::repositories::defs::user::{
    AddRoleToUserError, DeleteUserError, GetUserError, RemoveRoleFromUserError, UpdateUserError,
    UserRepository,
};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use std::sync::Arc;

pub async fn create_user(
    State(repo): State<Arc<impl UserRepository>>,
    Json(data): Json<UserDataDTO>,
) -> impl IntoResponse {
    match repo.create_user(data).await {
        Ok(user) => {
            println!("user is successfully created: {:?}", user);
            (StatusCode::CREATED, Json(user)).into_response()
        }
        Err(err) => {
            println!("error occurred when creating user: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message": "error occurred when creating user"
                })),
            )
                .into_response()
        }
    }
}

pub async fn update_user_name(
    State(repo): State<Arc<impl UserRepository>>,
    Path(id): Path<UserId>,
    Json(data): Json<UpdateUserNameDTO>,
) -> impl IntoResponse {
    match repo.update_user_name(&id, data).await {
        Ok(user) => {
            println!("user is successfully updated: {:?}", user);
            (StatusCode::OK, Json(user)).into_response()
        }
        Err(err) => {
            println!("error occurred when updating user: {:?}", err);
            match err {
                UpdateUserError::NotFound { id } => (
                    StatusCode::NOT_FOUND,
                    Json(json!({
                        "message": format!("user with id {} is not found", id)
                    })),
                )
                    .into_response(),
                UpdateUserError::Unknown(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "message": "error occured when updating user"
                    })),
                )
                    .into_response(),
            }
        }
    }
}

pub async fn delete_user(
    State(repo): State<Arc<impl UserRepository>>,
    Path(id): Path<UserId>,
) -> impl IntoResponse {
    match repo.delete_user(&id).await {
        Ok(deleted_user) => {
            println!("user is successfully deleted: {:?}", deleted_user);
            (StatusCode::OK, Json(deleted_user)).into_response()
        }
        Err(err) => {
            println!("error occurred when deleting user: {:?}", err);
            match err {
                DeleteUserError::NotFound { id } => (
                    StatusCode::NOT_FOUND,
                    Json(json!({
                        "message": format!("user with id {} is not found", id)
                    })),
                )
                    .into_response(),
                DeleteUserError::Unknown(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "message": "error occurred when deleting user"
                    })),
                )
                    .into_response(),
            }
        }
    }
}

pub async fn get_user_by_id(
    State(repo): State<Arc<impl UserRepository>>,
    Path(id): Path<UserId>,
) -> impl IntoResponse {
    match repo.get_user_by_id(&id).await {
        Ok(user) => {
            println!("user is successfully retrieved: {:?}", user);
            (StatusCode::OK, Json(user)).into_response()
        }
        Err(err) => {
            println!("error occurred when retrieving user: {:?}", err);
            match err {
                GetUserError::NotFound { id } => (
                    StatusCode::NOT_FOUND,
                    Json(json!({
                        "message": format!("user with id {} is not found", id)
                    })),
                )
                    .into_response(),
                GetUserError::Unknown(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "message": "error occurred when retrieving user"
                    })),
                )
                    .into_response(),
            }
        }
    }
}

pub async fn get_all_users(State(repo): State<Arc<impl UserRepository>>) -> impl IntoResponse {
    match repo.get_all_users().await {
        Ok(users) => {
            println!("users are successfully retrieved: {:?}", users);
            (
                StatusCode::OK,
                Json(json!({
                    "users": users
                })),
            )
                .into_response()
        }
        Err(err) => {
            println!("error occurred when retrieving users: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message": "error occurred when retrieving users"
                })),
            )
                .into_response()
        }
    }
}

pub async fn add_role_to_user(
    State(repo): State<Arc<impl UserRepository>>,
    Path(id): Path<UserId>,
    Path(slug): Path<RoleSlug>,
) -> impl IntoResponse {
    match repo.add_role_to_user(&id, &slug).await {
        Ok(_) => {
            println!("role is successfully added to user");
            StatusCode::OK.into_response()
        }
        Err(err) => {
            println!("error occurred when adding role to user: {:?}", err);
            match err {
                AddRoleToUserError::UserNotFound { id } => (
                    StatusCode::NOT_FOUND,
                    Json(json!({
                        "message": format!("user with id {} is not found", id)
                    })),
                )
                    .into_response(),
                AddRoleToUserError::RoleNotFound { slug } => (
                    StatusCode::NOT_FOUND,
                    Json(json!({
                        "message": format!("role with slug {} is not found", slug)
                    })),
                )
                    .into_response(),
                AddRoleToUserError::Unknown(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "message": "error occurred when adding role to user"
                    })),
                )
                    .into_response(),
            }
        }
    }
}

pub async fn remove_role_from_user(
    State(repo): State<Arc<impl UserRepository>>,
    Path(id): Path<UserId>,
    Path(slug): Path<RoleSlug>,
) -> impl IntoResponse {
    match repo.remove_role_from_user(&id, &slug).await {
        Ok(_) => {
            println!("role is successfully removed from user");
            StatusCode::OK.into_response()
        }
        Err(err) => {
            println!("error occurred when removing role from user: {:?}", err);
            match err {
                RemoveRoleFromUserError::UserNotFound { id } => (
                    StatusCode::NOT_FOUND,
                    Json(json!({
                        "message": format!("user with id {} is not found", id)
                    })),
                )
                    .into_response(),
                RemoveRoleFromUserError::RoleNotFound { slug } => (
                    StatusCode::NOT_FOUND,
                    Json(json!({
                        "message": format!("role with slug {} is not found", slug)
                    })),
                )
                    .into_response(),
                RemoveRoleFromUserError::Unknown(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "message": "error occurred when removing role from user"
                    })),
                )
                    .into_response(),
                RemoveRoleFromUserError::UserShouldHaveAtLeastOneRole => (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "message": "user should have at least one role"
                    })),
                )
                    .into_response(),
            }
        }
    }
}
