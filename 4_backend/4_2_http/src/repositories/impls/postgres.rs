use std::env;

use async_trait::async_trait;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::models::{
    AllUserRoles, GetUserResult, Role, RoleName, RolePermissions, User, UserData, UserId, UserName,
};
use crate::repositories::defs::role::RoleRepository;
use crate::repositories::defs::user::UserRepository;

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
    async fn create_user(&self, data: UserData) -> anyhow::Result<User> {
        Ok(sqlx::query_as!(
            User,
            r#"
            INSERT INTO "user"(name)
            VALUES ($1)
            RETURNING id, name
            "#,
            data.name
        )
        .fetch_one(&self.pool)
        .await?)
    }

    async fn update_user_name(&self, id: &UserId, name: UserName) -> anyhow::Result<User> {
        Ok(sqlx::query_as!(
            User,
            r#"
            UPDATE "user"
            SET name = $1
            WHERE id = $2
            RETURNING id, name
            "#,
            name,
            id
        )
        .fetch_one(&self.pool)
        .await?)
    }

    async fn delete_user(&self, id: &UserId) -> anyhow::Result<User> {
        Ok(sqlx::query_as!(
            User,
            r#"
            DELETE FROM "user"
            WHERE id = $1
            RETURNING id, name
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?)
    }

    async fn get_user_by_id(&self, id: &UserId) -> anyhow::Result<GetUserResult> {
        Ok(sqlx::query_as!(
            GetUserResult,
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
        .await?)
    }

    async fn get_all_users(&self) -> anyhow::Result<Vec<GetUserResult>> {
        Ok(sqlx::query_as!(
            GetUserResult,
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

    async fn add_role_to_user(&self, user_id: &UserId, role_slug: &str) -> anyhow::Result<()> {
        Ok(sqlx::query!(
            r#"
            INSERT INTO user_role(user_id, role_slug)
            VALUES ($1, $2)
            "#,
            user_id,
            role_slug
        )
        .execute(&self.pool)
        .await
        .map(|_| ())?)
    }

    async fn remove_role_from_user(&self, user_id: &UserId, role_slug: &str) -> anyhow::Result<()> {
        Ok(sqlx::query!(
            r#"
            DELETE FROM user_role
            WHERE user_id = $1 AND role_slug = $2
            "#,
            user_id,
            role_slug
        )
        .execute(&self.pool)
        .await
        .map(|_| ())?)
    }
}

#[async_trait]
impl RoleRepository for PostgresRepositoryImpl {
    async fn create_role(&self, data: Role) -> anyhow::Result<Role> {
        Ok(sqlx::query_as!(
            Role,
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

    async fn update_role_name(&self, slug: &str, name: RoleName) -> anyhow::Result<Role> {
        Ok(sqlx::query_as!(
            Role,
            r#"
            UPDATE role
            SET name = $1
            WHERE slug = $2
            RETURNING slug, name, permissions
            "#,
            name,
            slug
        )
        .fetch_one(&self.pool)
        .await?)
    }

    async fn update_role_permissions(
        &self,
        slug: &str,
        permissions: RolePermissions,
    ) -> anyhow::Result<Role> {
        Ok(sqlx::query_as!(
            Role,
            r#"
            UPDATE role
            SET permissions = $1
            WHERE slug = $2
            RETURNING slug, name, permissions
            "#,
            permissions,
            slug
        )
        .fetch_one(&self.pool)
        .await?)
    }

    async fn delete_role(&self, slug: &str) -> anyhow::Result<Role> {
        Ok(sqlx::query_as!(
            Role,
            r#"
            DELETE FROM role
            WHERE slug = $1
            RETURNING slug, name, permissions
            "#,
            slug
        )
        .fetch_one(&self.pool)
        .await?)
    }

    async fn get_role_by_slug(&self, slug: &str) -> anyhow::Result<Role> {
        Ok(sqlx::query_as!(
            Role,
            r#"
            SELECT slug, name, permissions
            FROM role
            WHERE slug = $1
            "#,
            slug
        )
        .fetch_one(&self.pool)
        .await?)
    }

    async fn get_all_roles(&self) -> anyhow::Result<Vec<Role>> {
        Ok(sqlx::query_as!(
            Role,
            r#"
            SELECT slug, name, permissions
            FROM role
            "#,
        )
        .fetch_all(&self.pool)
        .await?)
    }
}
