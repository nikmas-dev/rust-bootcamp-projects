use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use tracing::{error, info};

use crate::models::{RoleSlug, UpdateUserNameDTO, UserDataDTO, UserId};
use crate::repositories::defs::user::{
    AddRoleToUserError, CreateUserError, DeleteRoleFromUserError, DeleteUserError,
    GetAllUsersError, GetUserError, UpdateUserError, UserRepository,
};

impl IntoResponse for CreateUserError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "message": "error occurred when creating user"
            })),
        )
            .into_response()
    }
}

impl IntoResponse for UpdateUserError {
    fn into_response(self) -> Response {
        match self {
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

impl IntoResponse for DeleteUserError {
    fn into_response(self) -> Response {
        match self {
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

impl IntoResponse for GetUserError {
    fn into_response(self) -> Response {
        match self {
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

impl IntoResponse for GetAllUsersError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "message": "error occurred when retrieving users"
            })),
        )
            .into_response()
    }
}

impl IntoResponse for AddRoleToUserError {
    fn into_response(self) -> Response {
        match self {
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

impl IntoResponse for DeleteRoleFromUserError {
    fn into_response(self) -> Response {
        match self {
            DeleteRoleFromUserError::UserNotFound { id } => (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "message": format!("user with id {} is not found", id)
                })),
            )
                .into_response(),
            DeleteRoleFromUserError::RoleNotFound { slug } => (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "message": format!("role with slug {} is not found", slug)
                })),
            )
                .into_response(),
            DeleteRoleFromUserError::Unknown(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message": "error occurred when removing role from user"
                })),
            )
                .into_response(),
            DeleteRoleFromUserError::UserShouldHaveAtLeastOneRole => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "message": "user should have at least one role"
                })),
            )
                .into_response(),
        }
    }
}

pub async fn create_user<R: UserRepository>(data: UserDataDTO, repo: Arc<R>) -> impl IntoResponse {
    match repo.create_user(data).await {
        Ok(user) => {
            info!("user is successfully created: {:?}", user);
            (StatusCode::CREATED, Json(user)).into_response()
        }
        Err(err) => {
            error!("error occurred when creating user: {:?}", err);
            err.into_response()
        }
    }
}

pub async fn update_user_name<R: UserRepository>(
    id: &UserId,
    data: UpdateUserNameDTO,
    repo: Arc<R>,
) -> impl IntoResponse {
    match repo.update_user_name(&id, data).await {
        Ok(user) => {
            info!("user is successfully updated: {:?}", user);
            (StatusCode::OK, Json(user)).into_response()
        }
        Err(err) => {
            error!("error occurred when updating user: {:?}", err);
            err.into_response()
        }
    }
}

pub async fn delete_user<R: UserRepository>(id: &UserId, repo: Arc<R>) -> impl IntoResponse {
    match repo.delete_user(&id).await {
        Ok(deleted_user) => {
            info!("user is successfully deleted: {:?}", deleted_user);
            (StatusCode::OK, Json(deleted_user)).into_response()
        }
        Err(err) => {
            error!("error occurred when deleting user: {:?}", err);
            err.into_response()
        }
    }
}

pub async fn get_user_by_id<R: UserRepository>(id: &UserId, repo: Arc<R>) -> impl IntoResponse {
    match repo.get_user_by_id(&id).await {
        Ok(user) => {
            info!("user is successfully retrieved: {:?}", user);
            (StatusCode::OK, Json(user)).into_response()
        }
        Err(err) => {
            error!("error occurred when retrieving user: {:?}", err);
            err.into_response()
        }
    }
}

pub async fn get_all_users<R: UserRepository>(repo: Arc<R>) -> impl IntoResponse {
    match repo.get_all_users().await {
        Ok(users) => {
            info!("users are successfully retrieved: {:?}", users);
            (
                StatusCode::OK,
                Json(json!({
                    "users": users
                })),
            )
                .into_response()
        }
        Err(err) => {
            error!("error occurred when retrieving users: {:?}", err);
            err.into_response()
        }
    }
}

pub async fn add_role_to_user<R: UserRepository>(
    id: &UserId,
    slug: &RoleSlug,
    repo: Arc<R>,
) -> impl IntoResponse {
    match repo.add_role_to_user(&id, &slug).await {
        Ok(_) => {
            info!("role is successfully added to user");
            StatusCode::OK.into_response()
        }
        Err(err) => {
            error!("error occurred when adding role to user: {:?}", err);
            err.into_response()
        }
    }
}

pub async fn remove_role_from_user<R: UserRepository>(
    id: &UserId,
    slug: &RoleSlug,
    repo: Arc<R>,
) -> impl IntoResponse {
    match repo.remove_role_from_user(&id, &slug).await {
        Ok(_) => {
            info!("role is successfully removed from user");
            StatusCode::OK.into_response()
        }
        Err(err) => {
            error!("error occurred when removing role from user: {:?}", err);
            err.into_response()
        }
    }
}
