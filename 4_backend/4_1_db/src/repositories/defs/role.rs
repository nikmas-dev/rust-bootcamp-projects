use async_trait::async_trait;

use crate::models::{Role, RoleName, RolePermissions};

#[async_trait]
pub trait RoleRepository {
    async fn create_role(&self, data: Role) -> anyhow::Result<Role>;
    async fn update_role_name(&self, slug: &str, new_name: RoleName) -> anyhow::Result<Role>;
    async fn update_role_permissions(
        &self,
        slug: &str,
        new_permissions: RolePermissions,
    ) -> anyhow::Result<Role>;
    async fn delete_role(&self, slug: &str) -> anyhow::Result<Role>;
    async fn get_role_by_slug(&self, slug: &str) -> anyhow::Result<Role>;
    async fn get_all_roles(&self) -> anyhow::Result<Vec<Role>>;
}
