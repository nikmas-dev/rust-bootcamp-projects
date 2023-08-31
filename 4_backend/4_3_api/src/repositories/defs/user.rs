use async_trait::async_trait;
use thiserror::Error;

use crate::models::{
    GetUserResultDTO, RoleSlug, UpdateUserNameDTO, UserDTO, UserDataDTO, UserId, UserName,
};

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

    #[error("user should have at least one role")]
    UserShouldHaveAtLeastOneRole,

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[async_trait]
pub trait UserRepository {
    async fn create_user(&self, data: UserDataDTO) -> anyhow::Result<UserDTO>;
    async fn update_user_name(
        &self,
        id: &UserId,
        data: UpdateUserNameDTO,
    ) -> Result<UserDTO, UpdateUserError>;
    async fn delete_user(&self, id: &UserId) -> Result<UserDTO, DeleteUserError>;

    async fn get_user_by_id(&self, id: &UserId) -> Result<GetUserResultDTO, GetUserError>;
    async fn get_all_users(&self) -> anyhow::Result<Vec<GetUserResultDTO>>;

    async fn add_role_to_user(
        &self,
        user_id: &UserId,
        role_slug: &str,
    ) -> Result<(), AddRoleToUserError>;
    async fn remove_role_from_user(
        &self,
        user_id: &UserId,
        role_slug: &str,
    ) -> Result<(), RemoveRoleFromUserError>;
}
