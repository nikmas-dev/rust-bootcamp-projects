use async_trait::async_trait;
use thiserror::Error;

use crate::models::{GetUserResult, RoleSlug, User, UserData, UserId, UserName};

#[derive(Error, Debug)]
pub enum UpdateUserError {
    #[error("user with id {id} not found")]
    NotFound { id: UserId },

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum DeleteUserError {
    #[error("user with id {id} not found")]
    NotFound { id: UserId },

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum GetUserError {
    #[error("user with id {id} not found")]
    NotFound { id: UserId },

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum AddRoleToUserError {
    #[error("user with id {id} not found")]
    UserNotFound { id: UserId },
    
    #[error("role with slug {slug} not found")]
    RoleNotFound { slug: RoleSlug },

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum RemoveRoleFromUserError {
    #[error("user with id {id} not found")]
    UserNotFound { id: UserId },

    #[error("role with slug {slug} not found")]
    RoleNotFound { slug: RoleSlug },

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[async_trait]
pub trait UserRepository {
    async fn create_user(&self, data: UserData) -> anyhow::Result<User>;
    async fn update_user_name(&self, id: &UserId, new_name: UserName) -> Result<User, UpdateUserError>;
    async fn delete_user(&self, id: &UserId) -> Result<User, DeleteUserError>;

    async fn get_user_by_id(&self, id: &UserId) -> Result<GetUserResult, GetUserError>;
    async fn get_all_users(&self) -> anyhow::Result<Vec<GetUserResult>>;

    async fn add_role_to_user(&self, user_id: &UserId, role_slug: &str) -> Result<(), AddRoleToUserError>;
    async fn remove_role_from_user(&self, user_id: &UserId, role_slug: &str) -> Result<(), RemoveRoleFromUserError>;
}
