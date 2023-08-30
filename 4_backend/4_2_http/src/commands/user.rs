use std::sync::Arc;
use crate::models::{GetUserResult, User, UserData, UserId, UserName};
use crate::repositories::defs::user::UserRepository;

pub async fn create_user(data: UserData, repo: Arc<impl UserRepository>) -> anyhow::Result<User> {
    repo.create_user(data).await
}

pub async fn update_user_name(
    id: &UserId,
    new_name: UserName,
    repo: Arc<impl UserRepository>,
) -> anyhow::Result<User> {
    repo.update_user_name(id, new_name).await
}

pub async fn delete_user(id: &UserId, repo: Arc<impl UserRepository>) -> anyhow::Result<User> {
    repo.delete_user(id).await
}

pub async fn get_user_by_id(
    id: &UserId,
    repo: Arc<impl UserRepository>,
) -> anyhow::Result<GetUserResult> {
    repo.get_user_by_id(id).await
}

pub async fn get_all_users(repo: Arc<impl UserRepository>) -> anyhow::Result<Vec<GetUserResult>> {
    repo.get_all_users().await
}

pub async fn add_role_to_user(
    user_id: &UserId,
    role_slug: &str,
    repo: Arc<impl UserRepository>,
) -> anyhow::Result<()> {
    repo.add_role_to_user(user_id, role_slug).await
}

pub async fn remove_role_from_user(
    user_id: &UserId,
    role_slug: &str,
    repo: Arc<impl UserRepository>,
) -> anyhow::Result<()> {
    repo.remove_role_from_user(user_id, role_slug).await
}
