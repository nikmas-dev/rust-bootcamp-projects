use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use tracing::{error, info};

use crate::models::{RoleDTO, RoleSlug, UpdateRoleNameDTO, UpdateRolePermissionsDTO};
use crate::repositories::defs::role::{
    CreateRoleError, DeleteRoleError, GetAllRolesError, GetRoleError, RoleRepository,
    UpdateRoleError,
};

impl IntoResponse for CreateRoleError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "message": "error occurred when creating role"
            })),
        )
            .into_response()
    }
}

impl IntoResponse for UpdateRoleError {
    fn into_response(self) -> Response {
        match self {
            UpdateRoleError::NotFound { slug } => (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "message": format!("role with slug {} is not found", slug)
                })),
            )
                .into_response(),
            UpdateRoleError::Unknown(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message": "error occured when updating role"
                })),
            )
                .into_response(),
        }
    }
}

impl IntoResponse for DeleteRoleError {
    fn into_response(self) -> Response {
        match self {
            DeleteRoleError::NotFound { slug } => (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "message": format!("role with slug {} is not found", slug)
                })),
            )
                .into_response(),
            DeleteRoleError::Unknown(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message": "error occurred when deleting role"
                })),
            )
                .into_response(),
            DeleteRoleError::InUse { slug } => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "message": format!("role with slug {} is in use", slug)
                })),
            )
                .into_response(),
        }
    }
}

impl IntoResponse for GetRoleError {
    fn into_response(self) -> Response {
        match self {
            GetRoleError::NotFound { slug } => (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "message": format!("role with slug {} is not found", slug)
                })),
            )
                .into_response(),
            GetRoleError::Unknown(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message": "error occurred when getting role"
                })),
            )
                .into_response(),
        }
    }
}

impl IntoResponse for GetAllRolesError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "message": "error occurred when getting roles"
            })),
        )
            .into_response()
    }
}

pub async fn create_role<R: RoleRepository>(data: RoleDTO, repo: Arc<R>) -> impl IntoResponse {
    match repo.create_role(data).await {
        Ok(role) => {
            info!("role is successfully created: {:?}", role);
            (StatusCode::CREATED, Json(role)).into_response()
        }
        Err(err) => {
            error!("error occurred when creating role: {:?}", err);
            err.into_response()
        }
    }
}

pub async fn update_role_name<R: RoleRepository>(
    slug: &RoleSlug,
    data: UpdateRoleNameDTO,
    repo: Arc<R>,
) -> impl IntoResponse {
    match repo.update_role_name(&slug, data).await {
        Ok(role) => {
            info!("role name is successfully updated: {:?}", role);
            (StatusCode::OK, Json(role)).into_response()
        }
        Err(err) => {
            error!("error occurred when updating role: {:?}", err);
            err.into_response()
        }
    }
}

pub async fn update_role_permissions<R: RoleRepository>(
    slug: &RoleSlug,
    data: UpdateRolePermissionsDTO,
    repo: Arc<R>,
) -> impl IntoResponse {
    match repo.update_role_permissions(&slug, data).await {
        Ok(role) => {
            info!("role permissions are successfully updated: {:?}", role);
            (StatusCode::OK, Json(role)).into_response()
        }
        Err(err) => {
            error!("error occurred when updating role: {:?}", err);
            err.into_response()
        }
    }
}

pub async fn delete_role<R: RoleRepository>(slug: &RoleSlug, repo: Arc<R>) -> impl IntoResponse {
    match repo.delete_role(&slug).await {
        Ok(deleted_role) => {
            info!("role is successfully deleted: {:?}", deleted_role);
            (StatusCode::OK, Json(deleted_role)).into_response()
        }
        Err(err) => {
            error!("error occurred when deleting role: {:?}", err);
            err.into_response()
        }
    }
}

pub async fn get_role_by_slug<R: RoleRepository>(
    slug: &RoleSlug,
    repo: Arc<R>,
) -> impl IntoResponse {
    match repo.get_role_by_slug(&slug).await {
        Ok(role) => {
            info!("role is successfully retrieved: {:?}", role);
            (StatusCode::OK, Json(role)).into_response()
        }
        Err(err) => {
            error!("error occurred when getting role: {:?}", err);
            err.into_response()
        }
    }
}

pub async fn get_all_roles<R: RoleRepository>(repo: Arc<R>) -> impl IntoResponse {
    match repo.get_all_roles().await {
        Ok(roles) => {
            info!("roles are successfully retrieved: {:?}", roles);
            (
                StatusCode::OK,
                Json(json!({
                    "roles": roles
                })),
            )
                .into_response()
        }
        Err(err) => {
            error!("error occurred when getting roles: {:?}", err);
            err.into_response()
        }
    }
}
