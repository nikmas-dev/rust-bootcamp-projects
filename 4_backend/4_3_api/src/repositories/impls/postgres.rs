use std::env;

use async_trait::async_trait;
use sqlx::error::{DatabaseError, ErrorKind};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, PgPool};

use crate::constants::DEFAULT_ROLE_SLUG;
use crate::models::{
    GetUserResultDTO, RoleDTO, RoleName, RoleSlug, UpdateRoleNameDTO, UpdateRolePermissionsDTO,
    UpdateUserNameDTO, UserDTO, UserDataDTO, UserId,
};
use crate::repositories::defs::role::{
    CreateRoleError, DeleteRoleError, GetAllRolesError, GetRoleError, RoleRepository,
    UpdateRoleError,
};
use crate::repositories::defs::user::{
    AddRoleToUserError, CreateUserError, DeleteRoleFromUserError, DeleteUserError,
    GetAllUsersError, GetUserError, UpdateUserError, UserRepository,
};

pub trait CombinedRepository: UserRepository + RoleRepository {}

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

impl CombinedRepository for PostgresRepositoryImpl {}

#[async_trait]
impl UserRepository for PostgresRepositoryImpl {
    async fn create_user(&self, data: UserDataDTO) -> Result<UserDTO, CreateUserError> {
        let transaction = self
            .pool
            .begin()
            .await
            .map_err(|err| CreateUserError::Unknown(err.into()))?;

        let user = sqlx::query_as!(
            UserDTO,
            r#"
            INSERT INTO "user"(name)
            VALUES ($1)
            RETURNING id, name
            "#,
            data.name.0
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| CreateUserError::Unknown(err.into()))?;

        self.add_role_to_user(&user.id, &DEFAULT_ROLE_SLUG.to_owned().into())
            .await
            .map_err(|err| CreateUserError::Unknown(err.into()))?;

        transaction
            .commit()
            .await
            .map_err(|err| CreateUserError::Unknown(err.into()))?;

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
            new_name.0,
            id.0
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
            id.0
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
                COALESCE(NULLIF(ARRAY_AGG(role.name), '{NULL}'), '{}') AS "roles!: Vec<RoleName>"
            FROM "user"
                 LEFT JOIN user_role ON "user".id = user_role.user_id
                 LEFT JOIN role ON user_role.role_slug = role.slug
            WHERE "user".id = $1
            GROUP BY "user".id, "user".name
            "#,
            id.0
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| match err {
            Error::RowNotFound => GetUserError::NotFound { id: id.clone() },
            _ => GetUserError::Unknown(err.into()),
        })
    }

    async fn get_all_users(&self) -> Result<Vec<GetUserResultDTO>, GetAllUsersError> {
        sqlx::query_as!(
            GetUserResultDTO,
            r#"
            SELECT
                "user".id AS id,
                "user".name AS name,
                COALESCE(NULLIF(ARRAY_AGG(role.name), '{NULL}'), '{}') AS "roles!: Vec<RoleName>"
            FROM "user"
                 LEFT JOIN user_role ON "user".id = user_role.user_id
                 LEFT JOIN role ON user_role.role_slug = role.slug
            GROUP BY "user".id, "user".name
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|err| GetAllUsersError::Unknown(err.into()))
    }

    async fn add_role_to_user(
        &self,
        user_id: &UserId,
        role_slug: &RoleSlug,
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
            user_id.0,
            role_slug.0
        )
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(|err| AddRoleToUserError::Unknown(err.into()))
    }

    async fn remove_role_from_user(
        &self,
        user_id: &UserId,
        role_slug: &RoleSlug,
    ) -> Result<(), DeleteRoleFromUserError> {
        match self.get_user_by_id(user_id).await {
            Ok(user) => {
                if user.roles.len() == 1 {
                    return Err(DeleteRoleFromUserError::UserShouldHaveAtLeastOneRole);
                }
            }
            Err(GetUserError::NotFound { id }) => {
                return Err(DeleteRoleFromUserError::UserNotFound { id });
            }
            _ => (),
        }

        if let Err(GetRoleError::NotFound { slug }) = self.get_role_by_slug(role_slug).await {
            return Err(DeleteRoleFromUserError::RoleNotFound { slug });
        }

        sqlx::query!(
            r#"
            DELETE FROM user_role
            WHERE user_id = $1 AND role_slug = $2
            "#,
            user_id.0,
            role_slug.0
        )
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(|err| DeleteRoleFromUserError::Unknown(err.into()))
    }
}

#[async_trait]
impl RoleRepository for PostgresRepositoryImpl {
    async fn create_role(&self, data: RoleDTO) -> Result<RoleDTO, CreateRoleError> {
        sqlx::query_as!(
            RoleDTO,
            r#"
            INSERT INTO role(slug, name, permissions)
            VALUES ($1, $2, $3)
            RETURNING slug, name, permissions
            "#,
            data.slug.0,
            data.name.0,
            data.permissions.0
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| CreateRoleError::Unknown(err.into()))
    }

    async fn update_role_name(
        &self,
        slug: &RoleSlug,
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
            new_name.0,
            slug.0
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
        slug: &RoleSlug,
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
            new_permissions.0,
            slug.0
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

    async fn delete_role(&self, slug: &RoleSlug) -> Result<RoleDTO, DeleteRoleError> {
        sqlx::query_as!(
            RoleDTO,
            r#"
            DELETE FROM role
            WHERE slug = $1
            RETURNING slug, name, permissions
            "#,
            slug.0
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

    async fn get_role_by_slug(&self, slug: &RoleSlug) -> Result<RoleDTO, GetRoleError> {
        sqlx::query_as!(
            RoleDTO,
            r#"
            SELECT slug, name, permissions
            FROM role
            WHERE slug = $1
            "#,
            slug.0
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

    async fn get_all_roles(&self) -> Result<Vec<RoleDTO>, GetAllRolesError> {
        sqlx::query_as!(
            RoleDTO,
            r#"
            SELECT slug, name, permissions
            FROM role
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|err| GetAllRolesError::Unknown(err.into()))
    }
}
