use std::sync::Arc;

use axum::extract::{Path, State};
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

#[utoipa::path(
    post,
    path = "/api/users",
    request_body = UserDataDTO,
    responses(
        (status = 201, description = "user is successfully created", body = GetUserResultDTO)
    )
)]
pub async fn create_user<R: UserRepository>(
    State(repo): State<Arc<R>>,
    Json(data): Json<UserDataDTO>,
) -> impl IntoResponse {
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

#[utoipa::path(
    put,
    path = "/api/users/{id}/name",
    params(
        ("id" = UserId, Path, description = "user id")
    ),
    request_body = UpdateUserNameDTO,
    responses(
        (status = 200, description = "user name is successfully updated", body = UserDTO),
        (status = 404, description = "user not found")
    )
)]
pub async fn update_user_name<R: UserRepository>(
    State(repo): State<Arc<R>>,
    Path(id): Path<UserId>,
    Json(data): Json<UpdateUserNameDTO>,
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

#[utoipa::path(
    delete,
    path = "/api/users/{id}",
    params(
        ("id" = UserId, Path, description = "user id")
    ),
    responses(
        (status = 200, description = "user is successfully deleted", body = UserDTO),
        (status = 404, description = "user not found")
    )
)]
pub async fn delete_user<R: UserRepository>(
    State(repo): State<Arc<R>>,
    Path(id): Path<UserId>,
) -> impl IntoResponse {
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

#[utoipa::path(
    get,
    path = "/api/users/{id}",
    params(
        ("id" = UserId, Path, description = "user id")
    ),
    responses(
        (status = 200, description = "user is found", body = GetUserResultDTO),
        (status = 404, description = "user not found")
    )
)]
pub async fn get_user_by_id<R: UserRepository>(
    State(repo): State<Arc<R>>,
    Path(id): Path<UserId>,
) -> impl IntoResponse {
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

#[utoipa::path(
    get,
    path = "/api/users",
    responses(
        (status = 200, description = "users are successfully retrieved", body = Vec<GetUserResultDTO>),
    )
)]
pub async fn get_all_users<R: UserRepository>(State(repo): State<Arc<R>>) -> impl IntoResponse {
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

#[utoipa::path(
    post,
    path = "/api/users/{user_id}/roles/{role_slug}",
    params(
        ("user_id" = UserId, Path, description = "user id"),
        ("role slug" = RoleSlug, Path, description = "role slug"),
    ),
    responses(
        (status = 200, description = "role is successfully added to user"),
        (status = 404, description = "user not found"),
        (status = 404, description = "role not found"),
    )
)]
pub async fn add_role_to_user<R: UserRepository>(
    State(repo): State<Arc<R>>,
    Path(id): Path<UserId>,
    Path(slug): Path<RoleSlug>,
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

#[utoipa::path(
    delete,
    path = "/api/users/{user_id}/roles/{role_slug}",
    params(
        ("user_id" = UserId, Path, description = "user id"),
        ("role slug" = RoleSlug, Path, description = "role slug"),
    ),
    responses(
        (status = 200, description = "role is successfully removed from user"),
        (status = 404, description = "user not found"),
        (status = 404, description = "role not found"),
    )
)]
pub async fn remove_role_from_user<R: UserRepository>(
    State(repo): State<Arc<R>>,
    Path(id): Path<UserId>,
    Path(slug): Path<RoleSlug>,
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
