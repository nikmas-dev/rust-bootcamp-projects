use async_trait::async_trait;
use thiserror::Error;

use crate::models::{
    RoleDTO, RoleName, RolePermissions, RoleSlug, UpdateRoleNameDTO, UpdateRolePermissionsDTO,
};

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

#[async_trait]
pub trait RoleRepository {
    async fn create_role(&self, data: RoleDTO) -> anyhow::Result<RoleDTO>;
    async fn update_role_name(
        &self,
        slug: &str,
        data: UpdateRoleNameDTO,
    ) -> Result<RoleDTO, UpdateRoleError>;
    async fn update_role_permissions(
        &self,
        slug: &str,
        data: UpdateRolePermissionsDTO,
    ) -> Result<RoleDTO, UpdateRoleError>;
    async fn delete_role(&self, slug: &str) -> Result<RoleDTO, DeleteRoleError>;
    async fn get_role_by_slug(&self, slug: &str) -> Result<RoleDTO, GetRoleError>;
    async fn get_all_roles(&self) -> anyhow::Result<Vec<RoleDTO>>;
}
