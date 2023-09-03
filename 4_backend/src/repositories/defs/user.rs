use crate::models::{CreateUserDTO, FullUserDTO, UserDTO, UserId, UserName};
use async_trait::async_trait;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GetUserByIdError {
    #[error("user with id {id} not found")]
    NotFound { id: UserId },

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum GetUserByNameError {
    #[error("user with name {name} not found")]
    NotFound { name: UserName },

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum CreateUserError {
    #[error("user with name {name} already exists")]
    AlreadyExists { name: UserName },

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum AddFriendToUserError {
    #[error("user with id {user_id} not found")]
    UserNotFound { user_id: UserId },

    #[error("friend with name {friend_name} not found")]
    FriendNotFound { friend_name: UserName },

    #[error("user with id {user_id} already has friend with id {friend_id}")]
    UserAlreadyHasThisFriend { user_id: UserId, friend_id: UserId },

    #[error("user cannot be friend of himself")]
    UserCannotBeFriendOfHimself,

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum RemoveFriendFromUserError {
    #[error("user with id {user_id} not found")]
    UserNotFound { user_id: UserId },

    #[error("friend with name {friend_name} not found")]
    FriendNotFound { friend_name: UserName },

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn get_user_by_id(&self, id: &UserId) -> Result<FullUserDTO, GetUserByIdError>;
    async fn get_user_by_name(&self, name: &str) -> Result<FullUserDTO, GetUserByNameError>;
    async fn create_user(&self, data: CreateUserDTO) -> Result<UserDTO, CreateUserError>;
    async fn add_friend_to_user(
        &self,
        user_id: &UserId,
        friend_name: &str,
    ) -> Result<(), AddFriendToUserError>;
    async fn remove_friend_from_user(
        &self,
        user_id: &UserId,
        friend_name: &str,
    ) -> Result<(), RemoveFriendFromUserError>;
}
