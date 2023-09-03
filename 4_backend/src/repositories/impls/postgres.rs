use std::cmp::Ordering;
use std::env;

use async_trait::async_trait;
use axum::body::HttpBody;
use sqlx::error::ErrorKind;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, PgPool};

use crate::models::{CreateUserDTO, FullUserDTO, UserDTO, UserId, UserName, UserNoFriendsDTO};
use crate::repositories::defs::user::{
    AddFriendToUserError, CreateUserError, GetUserByIdError, GetUserByNameError,
    RemoveFriendFromUserError, UserRepository,
};

pub const MAX_POOL_SIZE: u32 = 20;
const DATABASE_URL_ENV: &str = "DATABASE_URL";

pub struct PostgresRepositoryImpl {
    pool: PgPool,
}

impl PostgresRepositoryImpl {
    pub async fn new() -> anyhow::Result<Self> {
        let db_url = env::var(DATABASE_URL_ENV)?;

        let pool = PgPoolOptions::new()
            .max_connections(MAX_POOL_SIZE)
            .connect(&db_url)
            .await?;

        Ok(Self { pool })
    }
}

#[async_trait]
impl UserRepository for PostgresRepositoryImpl {
    async fn get_user_by_id(&self, id: &UserId) -> Result<FullUserDTO, GetUserByIdError> {
        sqlx::query_as!(
            FullUserDTO,
            r#"
            SELECT
                u.id AS id,
                u.name AS name,
                u.password_hash AS password_hash,
                COALESCE(NULLIF(ARRAY_AGG(distinct friends.name), '{NULL}'), '{}') AS "friends_names!: Vec<UserName>"
            FROM "user" u
                 LEFT JOIN (
                    SELECT user_id, friend_id FROM user_friends WHERE user_id = $1
                    UNION
                    SELECT friend_id AS user_id, user_id AS friend_id FROM user_friends WHERE friend_id = $1
                 ) AS all_friends ON u.id = all_friends.user_id
                 LEFT JOIN "user" AS friends ON all_friends.friend_id = friends.id
            WHERE u.id = $1
            GROUP BY u.id, u.name, u.password_hash
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| match err {
            Error::RowNotFound => GetUserByIdError::NotFound { id: *id },
            _ => GetUserByIdError::Unknown(err.into()),
        })
    }

    async fn get_user_by_name(&self, name: &str) -> Result<FullUserDTO, GetUserByNameError> {
        sqlx::query_as!(
            FullUserDTO,
            r#"
            SELECT
                u.id AS id,
                u.name AS name,
                u.password_hash AS password_hash,
                COALESCE(NULLIF(ARRAY_AGG(distinct friends.name), '{NULL}'), '{}') AS "friends_names!: Vec<UserName>"
            FROM "user" u
                 LEFT JOIN (
                    SELECT user_id, friend_id FROM user_friends WHERE user_id = (SELECT id FROM "user" WHERE name = $1)
                    UNION
                    SELECT friend_id AS user_id, user_id AS friend_id FROM user_friends WHERE friend_id = (SELECT id FROM "user" WHERE name = $1)
                 ) AS all_friends ON u.id = all_friends.user_id
                 LEFT JOIN "user" AS friends ON all_friends.friend_id = friends.id
            WHERE u.name = $1
            GROUP BY u.id, u.name, u.password_hash
            "#,
            name
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| match err {
            Error::RowNotFound => GetUserByNameError::NotFound { name: name.to_owned() },
            _ => GetUserByNameError::Unknown(err.into()),
        })
    }

    async fn create_user(&self, data: CreateUserDTO) -> Result<UserDTO, CreateUserError> {
        let user = sqlx::query_as!(
            UserNoFriendsDTO,
            r#"
            INSERT INTO "user"(name, password_hash)
            VALUES ($1, $2)
            RETURNING id, name, password_hash
            "#,
            data.name,
            data.password_hash
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| match &err {
            Error::Database(db_err) => match db_err.kind() {
                ErrorKind::UniqueViolation => CreateUserError::AlreadyExists { name: data.name },
                _ => CreateUserError::Unknown(err.into()),
            },
            _ => CreateUserError::Unknown(err.into()),
        })?;

        Ok(UserDTO {
            id: user.id,
            name: user.name,
            friends_names: Vec::new(),
        })
    }

    async fn add_friend_to_user(
        &self,
        user_id: &UserId,
        friend_name: &str,
    ) -> Result<(), AddFriendToUserError> {
        if let Err(GetUserByIdError::NotFound { id: user_id }) = self.get_user_by_id(user_id).await
        {
            return Err(AddFriendToUserError::UserNotFound { user_id });
        }

        let friend = self.get_user_by_name(friend_name).await.map_err(|_| {
            AddFriendToUserError::FriendNotFound {
                friend_name: friend_name.to_owned(),
            }
        })?;

        let (user_id_to_insert, friend_id_to_insert) = match user_id.cmp(&friend.id) {
            Ordering::Less => (*user_id, friend.id),
            Ordering::Greater => (friend.id, *user_id),
            Ordering::Equal => {
                return Err(AddFriendToUserError::UserCannotBeFriendOfHimself);
            }
        };

        sqlx::query!(
            r#"
            INSERT INTO user_friends(user_id, friend_id)
            VALUES ($1, $2)
            "#,
            user_id_to_insert,
            friend_id_to_insert
        )
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(|err| match &err {
            Error::Database(db_err) => match db_err.kind() {
                ErrorKind::UniqueViolation => AddFriendToUserError::UserAlreadyHasThisFriend {
                    user_id: *user_id,
                    friend_id: friend.id,
                },
                _ => AddFriendToUserError::Unknown(err.into()),
            },
            _ => AddFriendToUserError::Unknown(err.into()),
        })
    }

    async fn remove_friend_from_user(
        &self,
        user_id: &UserId,
        friend_name: &str,
    ) -> Result<(), RemoveFriendFromUserError> {
        if let Err(GetUserByIdError::NotFound { id: user_id }) = self.get_user_by_id(user_id).await
        {
            return Err(RemoveFriendFromUserError::UserNotFound { user_id });
        }

        let friend = self.get_user_by_name(friend_name).await.map_err(|_| {
            RemoveFriendFromUserError::FriendNotFound {
                friend_name: friend_name.to_owned(),
            }
        })?;

        let (user_id_to_delete, friend_id_to_delete) = {
            if user_id < &friend.id {
                (*user_id, friend.id)
            } else {
                (friend.id, *user_id)
            }
        };

        let result = sqlx::query!(
            r#"
            DELETE FROM user_friends
            WHERE user_id = $1 AND friend_id = $2
            "#,
            user_id_to_delete,
            friend_id_to_delete
        )
        .execute(&self.pool)
        .await
        .map_err(|err| RemoveFriendFromUserError::Unknown(err.into()))?;

        if result.rows_affected() == 0 {
            return Err(RemoveFriendFromUserError::FriendNotFound {
                friend_name: friend_name.to_owned(),
            });
        }

        Ok(())
    }
}
