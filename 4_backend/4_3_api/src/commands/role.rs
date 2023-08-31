use crate::models::{Role, RoleName, RolePermissions};
use crate::repositories::defs::role::{
    DeleteRoleError, GetRoleError, RoleRepository, UpdateRoleError,
};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use std::sync::Arc;

pub async fn create_role(data: Role, repo: Arc<impl RoleRepository>) -> impl IntoResponse {
    match repo.create_role(data).await {
        Ok(role) => {
            println!("role is successfully created: {:?}", role);
            (StatusCode::CREATED, Json(role)).into_response()
        }
        Err(err) => {
            println!("error occurred when creating role: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message": "error occurred when creating role"
                })),
            )
                .into_response()
        }
    }
}

pub async fn update_role_name(
    slug: &str,
    new_name: RoleName,
    repo: Arc<impl RoleRepository>,
) -> impl IntoResponse {
    match repo.update_role_name(slug, new_name).await {
        Ok(role) => {
            println!("role name is successfully updated: {:?}", role);
            (StatusCode::OK, Json(role)).into_response()
        }
        Err(err) => {
            println!("error occurred when updating role: {:?}", err);
            match err {
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
}

pub async fn update_role_permissions(
    slug: &str,
    new_permissions: RolePermissions,
    repo: Arc<impl RoleRepository>,
) -> impl IntoResponse {
    match repo.update_role_permissions(slug, new_permissions).await {
        Ok(role) => {
            println!("role permissions are successfully updated: {:?}", role);
            (StatusCode::OK, Json(role)).into_response()
        }
        Err(err) => {
            println!("error occurred when updating role: {:?}", err);
            match err {
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
                        "message": "error occurred when updating role"
                    })),
                )
                    .into_response(),
            }
        }
    }
}

pub async fn delete_role(slug: &str, repo: Arc<impl RoleRepository>) -> impl IntoResponse {
    match repo.delete_role(slug).await {
        Ok(deleted_role) => {
            println!("role is successfully deleted: {:?}", deleted_role);
            (StatusCode::OK, Json(deleted_role)).into_response()
        }
        Err(err) => {
            println!("error occurred when deleting role: {:?}", err);
            match err {
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
}

pub async fn get_role_by_slug(slug: &str, repo: Arc<impl RoleRepository>) -> impl IntoResponse {
    match repo.get_role_by_slug(slug).await {
        Ok(role) => {
            println!("role is successfully retrieved: {:?}", role);
            (StatusCode::OK, Json(role)).into_response()
        }
        Err(err) => {
            println!("error occurred when getting role: {:?}", err);
            match err {
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
}

pub async fn get_all_roles(repo: Arc<impl RoleRepository>) -> impl IntoResponse {
    match repo.get_all_roles().await {
        Ok(roles) => {
            println!("roles are successfully retrieved: {:?}", roles);
            (
                StatusCode::OK,
                Json(json!({
                    "roles": roles
                })),
            )
                .into_response()
        }
        Err(err) => {
            println!("error occurred when getting roles: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message": "error occurred when getting roles"
                })),
            )
                .into_response()
        }
    }
}
