use std::marker::PhantomData;
use std::sync::Arc;

use async_graphql::{Context, Object};

use crate::graphql::models::User;
use crate::models::{UserId, UserName};
use crate::repositories::defs::user::{GetUserByIdError, GetUserByNameError, UserRepository};
use crate::utils::auth::authorize_graphql;

pub struct UserQuery<R>(PhantomData<R>);

impl<T> Default for UserQuery<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

#[Object]
impl<R: UserRepository> UserQuery<R> {
    pub async fn me(&self, ctx: &Context<'_>) -> async_graphql::Result<User<R>> {
        let claims = authorize_graphql(ctx)?;

        let repo: &Arc<R> = ctx.data().unwrap();

        let user = repo.get_user_by_id(&claims.user_id).await.map_err(|err| {
            if let GetUserByIdError::Unknown(_) = err {
                return async_graphql::Error::new("internal server error");
            }

            err.into()
        })?;

        Ok(user.into())
    }
}

#[Object]
impl<R: UserRepository> User<R> {
    pub async fn friends(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<User<R>>> {
        let repo: &Arc<R> = ctx.data().unwrap();

        let mut friends = Vec::with_capacity(self.inner.friends_names.len());

        for friend_name in &self.inner.friends_names {
            let friend = repo.get_user_by_name(friend_name).await.map_err(|err| {
                if let GetUserByNameError::Unknown(_) = err {
                    return async_graphql::Error::new("internal server error");
                }

                err.into()
            })?;

            friends.push(friend.into());
        }

        Ok(friends)
    }

    pub async fn id(&self) -> &UserId {
        &self.inner.id
    }

    pub async fn name(&self) -> &UserName {
        &self.inner.name
    }

    pub async fn friends_names(&self) -> &[UserName] {
        &self.inner.friends_names
    }
}
