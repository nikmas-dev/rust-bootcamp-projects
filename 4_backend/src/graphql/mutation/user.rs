use std::marker::PhantomData;
use std::sync::Arc;

use crate::constants::JWT_EXPIRATION_HOURS;
use async_graphql::{Context, Object};
use bcrypt::DEFAULT_COST;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Header};

use crate::graphql::models::{
    AddFriendInput, AddFriendToUserResponse, LoginResponse, LoginUserInput, RegisterUserInput,
    RemoveFriendFromUserResponse, RemoveFriendInput, User,
};
use crate::models::{AuthToken, CreateUserDTO};
use crate::repositories::defs::user::{
    AddFriendToUserError, CreateUserError, GetUserByNameError, RemoveFriendFromUserError,
    UserRepository,
};
use crate::utils::auth::{authorize_graphql, AuthError, Claims, KEYS};

pub struct UserMutation<R>(PhantomData<R>);

impl<T> Default for UserMutation<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

#[Object]
impl<R: UserRepository> UserMutation<R> {
    pub async fn register_user(
        &self,
        ctx: &Context<'_>,
        input: RegisterUserInput,
    ) -> async_graphql::Result<User<R>> {
        let repo: &Arc<R> = ctx.data().unwrap();

        let password_hash = bcrypt::hash(input.password, DEFAULT_COST)?;

        repo.create_user(CreateUserDTO {
            name: input.name,
            password_hash,
        })
        .await
        .map(|user| user.into())
        .map_err(|err| {
            if let CreateUserError::Unknown(_) = err {
                return async_graphql::Error::new("internal server error");
            }

            err.into()
        })
    }

    pub async fn login(
        &self,
        ctx: &Context<'_>,
        input: LoginUserInput,
    ) -> async_graphql::Result<LoginResponse> {
        let repo: &Arc<R> = ctx.data().unwrap();

        let user = repo.get_user_by_name(&input.name).await.map_err(|err| {
            if let GetUserByNameError::Unknown(_) = err {
                return async_graphql::Error::new("internal server error");
            }

            err.into()
        })?;

        if bcrypt::verify(input.password, &user.password_hash)
            .map_err(|_| async_graphql::Error::new("internal server error"))?
        {
            let claims = Claims {
                user_id: user.id,
                exp: (Utc::now() + Duration::hours(JWT_EXPIRATION_HOURS as i64)).timestamp(),
            };

            let token = encode(&Header::default(), &claims, &KEYS.encoding)
                .map_err(|_| async_graphql::Error::new("internal server error"))?;

            return Ok(LoginResponse { token });
        }

        Err(AuthError::InvalidCredentials.into())
    }

    pub async fn add_friend(
        &self,
        ctx: &Context<'_>,
        input: AddFriendInput,
    ) -> async_graphql::Result<AddFriendToUserResponse> {
        let claims = authorize_graphql(ctx)?;

        let repo: &Arc<R> = ctx.data().unwrap();

        repo.add_friend_to_user(&claims.user_id, &input.friend_name)
            .await
            .map_err(|err| {
                if let AddFriendToUserError::Unknown(_) = err {
                    return async_graphql::Error::new("internal server error");
                }

                err.into()
            })?;

        Ok(AddFriendToUserResponse {
            message: format!(
                "friend with name {} is successfully added",
                input.friend_name
            ),
        })
    }

    pub async fn remove_friend(
        &self,
        ctx: &Context<'_>,
        input: RemoveFriendInput,
    ) -> async_graphql::Result<RemoveFriendFromUserResponse> {
        let claims = authorize_graphql(ctx)?;

        let repo: &Arc<R> = ctx.data().unwrap();

        repo.remove_friend_from_user(&claims.user_id, &input.friend_name)
            .await
            .map_err(|err| {
                if let RemoveFriendFromUserError::Unknown(_) = err {
                    return async_graphql::Error::new("internal server error");
                }

                err.into()
            })?;

        Ok(RemoveFriendFromUserResponse {
            message: format!(
                "friend with name {} is successfully removed",
                input.friend_name
            ),
        })
    }
}
