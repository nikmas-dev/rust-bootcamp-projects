use async_graphql::{InputObject, SimpleObject};
use std::marker::PhantomData;

use crate::models::{AuthToken, FullUserDTO, UserDTO, UserName, UserPassword};

pub struct User<R> {
    pub inner: UserDTO,
    pub phantom: PhantomData<R>,
}

impl<R> From<UserDTO> for User<R> {
    fn from(user: UserDTO) -> Self {
        Self {
            inner: user,
            phantom: PhantomData,
        }
    }
}

impl<R> From<FullUserDTO> for User<R> {
    fn from(user: FullUserDTO) -> Self {
        Self {
            inner: user.into(),
            phantom: PhantomData,
        }
    }
}

#[derive(SimpleObject)]
pub struct AddFriendToUserResponse {
    pub message: String,
}

#[derive(SimpleObject)]
pub struct RemoveFriendFromUserResponse {
    pub message: String,
}

#[derive(SimpleObject)]
pub struct LoginResponse {
    pub token: AuthToken,
}

#[derive(InputObject)]
pub struct RegisterUserInput {
    pub name: UserName,
    pub password: UserPassword,
}

#[derive(InputObject)]
pub struct LoginUserInput {
    pub name: UserName,
    pub password: UserPassword,
}

#[derive(InputObject)]
pub struct AddFriendInput {
    pub friend_name: UserName,
}

#[derive(InputObject)]
pub struct RemoveFriendInput {
    pub friend_name: UserName,
}
