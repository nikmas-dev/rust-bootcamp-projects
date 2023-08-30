use async_trait::async_trait;

use crate::models::{GetUserResult, User, UserData, UserId, UserName};

#[async_trait]
pub trait UserRepository {
    async fn create_user(&self, data: UserData) -> anyhow::Result<User>;
    async fn update_user_name(&self, id: &UserId, new_name: UserName) -> anyhow::Result<User>;
    async fn delete_user(&self, id: &UserId) -> anyhow::Result<User>;

    async fn get_user_by_id(&self, id: &UserId) -> anyhow::Result<GetUserResult>;
    async fn get_all_users(&self) -> anyhow::Result<Vec<GetUserResult>>;

    async fn add_role_to_user(&self, user_id: &UserId, role_slug: &str) -> anyhow::Result<()>;
    async fn remove_role_from_user(&self, user_id: &UserId, role_slug: &str) -> anyhow::Result<()>;
}
