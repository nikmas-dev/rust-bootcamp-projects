use async_trait::async_trait;
use derive_more::From;
use thiserror::Error;

use crate::models::{
    RoleDTO, RoleName, RolePermissions, RoleSlug, UpdateRoleNameDTO, UpdateRolePermissionsDTO,
};

#[derive(Error, Debug)]
pub enum CreateRoleError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum UpdateRoleError {
    #[error("role with slug {slug} not found")]
    NotFound { slug: RoleSlug },

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum DeleteRoleError {
    #[error("role with slug {slug} not found")]
    NotFound { slug: RoleSlug },

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),

    #[error("role with slug {slug} is in use")]
    InUse { slug: RoleSlug },
}

#[derive(Error, Debug)]
pub enum GetRoleError {
    #[error("role with slug {slug} not found")]
    NotFound { slug: RoleSlug },

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum GetAllRolesError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[async_trait]
pub trait RoleRepository: Send + Sync + Sized {
    async fn create_role(&self, data: RoleDTO) -> Result<RoleDTO, CreateRoleError>;
    async fn update_role_name(
        &self,
        slug: &RoleSlug,
        data: UpdateRoleNameDTO,
    ) -> Result<RoleDTO, UpdateRoleError>;
    async fn update_role_permissions(
        &self,
        slug: &RoleSlug,
        data: UpdateRolePermissionsDTO,
    ) -> Result<RoleDTO, UpdateRoleError>;
    async fn delete_role(&self, slug: &RoleSlug) -> Result<RoleDTO, DeleteRoleError>;
    async fn get_role_by_slug(&self, slug: &RoleSlug) -> Result<RoleDTO, GetRoleError>;
    async fn get_all_roles(&self) -> Result<Vec<RoleDTO>, GetAllRolesError>;
}
