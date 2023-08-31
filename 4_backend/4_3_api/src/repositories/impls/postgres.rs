use std::env;

use crate::constants::DEFAULT_ROLE_SLUG;
use async_trait::async_trait;
use sqlx::error::{DatabaseError, ErrorKind};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, PgPool};

use crate::models::{
    AllUserRoles, GetUserResultDTO, RoleDTO, RoleName, RolePermissions, UpdateRoleNameDTO,
    UpdateRolePermissionsDTO, UpdateUserNameDTO, UserDTO, UserDataDTO, UserId, UserName,
};
use crate::repositories::defs::role::{
    DeleteRoleError, GetRoleError, RoleRepository, UpdateRoleError,
};
use crate::repositories::defs::user::{
    AddRoleToUserError, DeleteUserError, GetUserError, RemoveRoleFromUserError, UpdateUserError,
    UserRepository,
};

const MAX_POOL_SIZE: u32 = 20;
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
    async fn create_user(&self, data: UserDataDTO) -> anyhow::Result<UserDTO> {
        let user = sqlx::query_as!(
            UserDTO,
            r#"
            INSERT INTO "user"(name)
            VALUES ($1)
            RETURNING id, name
            "#,
            data.name
        )
        .fetch_one(&self.pool)
        .await?;

        self.add_role_to_user(&user.id, DEFAULT_ROLE_SLUG).await?;

        Ok(user)
    }

    async fn update_user_name(
        &self,
        id: &UserId,
        UpdateUserNameDTO { new_name }: UpdateUserNameDTO,
    ) -> Result<UserDTO, UpdateUserError> {
        sqlx::query_as!(
            UserDTO,
            r#"
            UPDATE "user"
            SET name = $1
            WHERE id = $2
            RETURNING id, name
            "#,
            new_name,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| match err {
            Error::RowNotFound => UpdateUserError::NotFound { id: id.clone() },
            _ => UpdateUserError::Unknown(err.into()),
        })
    }

    async fn delete_user(&self, id: &UserId) -> Result<UserDTO, DeleteUserError> {
        sqlx::query_as!(
            UserDTO,
            r#"
            DELETE FROM "user"
            WHERE id = $1
            RETURNING id, name
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| match err {
            Error::RowNotFound => DeleteUserError::NotFound { id: id.clone() },
            _ => DeleteUserError::Unknown(err.into()),
        })
    }

    async fn get_user_by_id(&self, id: &UserId) -> Result<GetUserResultDTO, GetUserError> {
        sqlx::query_as!(
            GetUserResultDTO,
            r#"
            SELECT
                "user".id AS id,
                "user".name AS name,
                COALESCE(NULLIF(ARRAY_AGG(role.name), '{NULL}'), '{}') AS "roles!: AllUserRoles"
            FROM "user"
                 LEFT JOIN user_role ON "user".id = user_role.user_id
                 LEFT JOIN role ON user_role.role_slug = role.slug
            WHERE "user".id = $1
            GROUP BY "user".id, "user".name
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| match err {
            Error::RowNotFound => GetUserError::NotFound { id: id.clone() },
            _ => GetUserError::Unknown(err.into()),
        })
    }

    async fn get_all_users(&self) -> anyhow::Result<Vec<GetUserResultDTO>> {
        Ok(sqlx::query_as!(
            GetUserResultDTO,
            r#"
            SELECT
                "user".id AS id,
                "user".name AS name,
                COALESCE(NULLIF(ARRAY_AGG(role.name), '{NULL}'), '{}') AS "roles!: AllUserRoles"
            FROM "user"
                 LEFT JOIN user_role ON "user".id = user_role.user_id
                 LEFT JOIN role ON user_role.role_slug = role.slug
            GROUP BY "user".id, "user".name
            "#,
        )
        .fetch_all(&self.pool)
        .await?)
    }

    async fn add_role_to_user(
        &self,
        user_id: &UserId,
        role_slug: &str,
    ) -> Result<(), AddRoleToUserError> {
        if let Err(GetUserError::NotFound { id }) = self.get_user_by_id(user_id).await {
            return Err(AddRoleToUserError::UserNotFound { id });
        }

        if let Err(GetRoleError::NotFound { slug }) = self.get_role_by_slug(role_slug).await {
            return Err(AddRoleToUserError::RoleNotFound { slug });
        }

        sqlx::query!(
            r#"
            INSERT INTO user_role(user_id, role_slug)
            VALUES ($1, $2)
            "#,
            user_id,
            role_slug
        )
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(|err| AddRoleToUserError::Unknown(err.into()))
    }

    async fn remove_role_from_user(
        &self,
        user_id: &UserId,
        role_slug: &str,
    ) -> Result<(), RemoveRoleFromUserError> {
        match self.get_user_by_id(user_id).await {
            Ok(user) => {
                if user.roles.len() == 1 {
                    return Err(RemoveRoleFromUserError::UserShouldHaveAtLeastOneRole);
                }
            }
            Err(GetUserError::NotFound { id }) => {
                return Err(RemoveRoleFromUserError::UserNotFound { id });
            }
            _ => (),
        }

        if let Err(GetRoleError::NotFound { slug }) = self.get_role_by_slug(role_slug).await {
            return Err(RemoveRoleFromUserError::RoleNotFound { slug });
        }

        sqlx::query!(
            r#"
            DELETE FROM user_role
            WHERE user_id = $1 AND role_slug = $2
            "#,
            user_id,
            role_slug
        )
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(|err| RemoveRoleFromUserError::Unknown(err.into()))
    }
}

#[async_trait]
impl RoleRepository for PostgresRepositoryImpl {
    async fn create_role(&self, data: RoleDTO) -> anyhow::Result<RoleDTO> {
        Ok(sqlx::query_as!(
            RoleDTO,
            r#"
            INSERT INTO role(slug, name, permissions)
            VALUES ($1, $2, $3)
            RETURNING slug, name, permissions
            "#,
            data.slug,
            data.name,
            data.permissions
        )
        .fetch_one(&self.pool)
        .await?)
    }

    async fn update_role_name(
        &self,
        slug: &str,
        UpdateRoleNameDTO { new_name }: UpdateRoleNameDTO,
    ) -> Result<RoleDTO, UpdateRoleError> {
        sqlx::query_as!(
            RoleDTO,
            r#"
            UPDATE role
            SET name = $1
            WHERE slug = $2
            RETURNING slug, name, permissions
            "#,
            new_name,
            slug
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| match err {
            Error::RowNotFound => UpdateRoleError::NotFound {
                slug: slug.to_owned(),
            },
            _ => UpdateRoleError::Unknown(err.into()),
        })
    }

    async fn update_role_permissions(
        &self,
        slug: &str,
        UpdateRolePermissionsDTO { new_permissions }: UpdateRolePermissionsDTO,
    ) -> Result<RoleDTO, UpdateRoleError> {
        sqlx::query_as!(
            RoleDTO,
            r#"
            UPDATE role
            SET permissions = $1
            WHERE slug = $2
            RETURNING slug, name, permissions
            "#,
            new_permissions,
            slug
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| match err {
            Error::RowNotFound => UpdateRoleError::NotFound {
                slug: slug.to_owned(),
            },
            _ => UpdateRoleError::Unknown(err.into()),
        })
    }

    async fn delete_role(&self, slug: &str) -> Result<RoleDTO, DeleteRoleError> {
        sqlx::query_as!(
            RoleDTO,
            r#"
            DELETE FROM role
            WHERE slug = $1
            RETURNING slug, name, permissions
            "#,
            slug
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| match err {
            Error::RowNotFound => DeleteRoleError::NotFound {
                slug: slug.to_owned(),
            },
            Error::Database(err) => match err.kind() {
                ErrorKind::ForeignKeyViolation => DeleteRoleError::InUse {
                    slug: slug.to_owned(),
                },
                _ => DeleteRoleError::Unknown(err.into()),
            },
            _ => DeleteRoleError::Unknown(err.into()),
        })
    }

    async fn get_role_by_slug(&self, slug: &str) -> Result<RoleDTO, GetRoleError> {
        sqlx::query_as!(
            RoleDTO,
            r#"
            SELECT slug, name, permissions
            FROM role
            WHERE slug = $1
            "#,
            slug
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| match err {
            Error::RowNotFound => GetRoleError::NotFound {
                slug: slug.to_owned(),
            },
            _ => GetRoleError::Unknown(err.into()),
        })
    }

    async fn get_all_roles(&self) -> anyhow::Result<Vec<RoleDTO>> {
        Ok(sqlx::query_as!(
            RoleDTO,
            r#"
            SELECT slug, name, permissions
            FROM role
            "#,
        )
        .fetch_all(&self.pool)
        .await?)
    }
}
