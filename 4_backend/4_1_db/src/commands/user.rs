use crate::models::{GetUserResult, User, UserData, UserId, UserName};
use crate::repositories::defs::role::DeleteRoleError;
use crate::repositories::defs::user::{
    AddRoleToUserError, DeleteUserError, GetUserError, RemoveRoleFromUserError, UpdateUserError,
    UserRepository,
};

pub async fn create_user(data: UserData, repo: &impl UserRepository) -> anyhow::Result<User> {
    repo.create_user(data).await
}

pub async fn update_user_name(
    id: &UserId,
    new_name: UserName,
    repo: &impl UserRepository,
) -> Result<User, UpdateUserError> {
    repo.update_user_name(id, new_name).await
}

pub async fn delete_user(id: &UserId, repo: &impl UserRepository) -> Result<User, DeleteUserError> {
    repo.delete_user(id).await
}

pub async fn get_user_by_id(
    id: &UserId,
    repo: &impl UserRepository,
) -> Result<GetUserResult, GetUserError> {
    repo.get_user_by_id(id).await
}

pub async fn get_all_users(repo: &impl UserRepository) -> anyhow::Result<Vec<GetUserResult>> {
    repo.get_all_users().await
}

pub async fn add_role_to_user(
    user_id: &UserId,
    role_slug: &str,
    repo: &impl UserRepository,
) -> Result<(), AddRoleToUserError> {
    repo.add_role_to_user(user_id, role_slug).await
}

pub async fn remove_role_from_user(
    user_id: &UserId,
    role_slug: &str,
    repo: &impl UserRepository,
) -> anyhow::Result<(), RemoveRoleFromUserError> {
    repo.remove_role_from_user(user_id, role_slug).await
}
