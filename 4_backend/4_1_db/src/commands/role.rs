use crate::models::{Role, RoleName, RolePermissions};
use crate::repositories::defs::role::RoleRepository;

pub async fn create_role(data: Role, repo: &impl RoleRepository) -> anyhow::Result<Role> {
    repo.create_role(data).await
}

pub async fn update_role_name(
    slug: &str,
    new_name: RoleName,
    repo: &impl RoleRepository,
) -> anyhow::Result<Role> {
    repo.update_role_name(slug, new_name).await
}

pub async fn update_role_permissions(
    slug: &str,
    new_permissions: RolePermissions,
    repo: &impl RoleRepository,
) -> anyhow::Result<Role> {
    repo.update_role_permissions(slug, new_permissions).await
}

pub async fn delete_role(slug: &str, repo: &impl RoleRepository) -> anyhow::Result<Role> {
    repo.delete_role(slug).await
}

pub async fn get_role_by_slug(slug: &str, repo: &impl RoleRepository) -> anyhow::Result<Role> {
    repo.get_role_by_slug(slug).await
}

pub async fn get_all_roles(repo: &impl RoleRepository) -> anyhow::Result<Vec<Role>> {
    repo.get_all_roles().await
}
