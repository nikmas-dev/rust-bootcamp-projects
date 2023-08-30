use async_trait::async_trait;
use thiserror::Error;

use crate::models::{Role, RoleName, RolePermissions, RoleSlug};

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
    async fn create_role(&self, data: Role) -> anyhow::Result<Role>;
    async fn update_role_name(
        &self,
        slug: &str,
        new_name: RoleName,
    ) -> Result<Role, UpdateRoleError>;
    async fn update_role_permissions(
        &self,
        slug: &str,
        new_permissions: RolePermissions,
    ) -> Result<Role, UpdateRoleError>;
    async fn delete_role(&self, slug: &str) -> Result<Role, DeleteRoleError>;
    async fn get_role_by_slug(&self, slug: &str) -> Result<Role, GetRoleError>;
    async fn get_all_roles(&self) -> anyhow::Result<Vec<Role>>;
}
