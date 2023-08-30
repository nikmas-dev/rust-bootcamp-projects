use clap::{Parser, Subcommand};
use sqlx::postgres::PgPoolOptions;
use std::env;

type UserName = String;
type UserId = i64;
type RoleSlug = String;
type RoleName = String;
type RolePermissions = String;

#[derive(Parser)]
#[command(about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(subcommand)]
    User(UserCommand),
    #[command(subcommand)]
    Role(RoleCommand),
}

#[derive(Subcommand)]
enum UserCommand {
    Create {
        #[arg(long)]
        name: UserName,
    },
    UpdateName {
        #[arg(long)]
        id: UserId,
        #[arg(long)]
        new_name: UserName,
    },
    Delete {
        #[arg(long)]
        id: UserId,
    },
    GetById {
        id: UserId,
    },
    GetAll,
    AddRole {
        #[arg(long)]
        user_id: UserId,
        #[arg(long)]
        role_slug: RoleSlug,
    },
    RemoveRole {
        #[arg(long)]
        user_id: UserId,
        #[arg(long)]
        role_slug: RoleSlug,
    },
}

#[derive(Subcommand)]
enum RoleCommand {
    Create {
        #[arg(long)]
        slug: RoleSlug,
        #[arg(long)]
        name: RoleName,
        #[arg(long)]
        permissions: RolePermissions,
    },
    UpdateName {
        #[arg(long)]
        slug: RoleSlug,
        #[arg(long)]
        new_name: RoleName,
    },
    UpdatePermissions {
        #[arg(long)]
        slug: RoleSlug,
        #[arg(long)]
        new_permissions: RolePermissions,
    },
    Delete {
        #[arg(long)]
        slug: RoleSlug,
    },
    GetBySlug {
        slug: RoleSlug,
    },
    GetAll,
}

#[tokio::main]
async fn main() {
    // let cli = Cli::parse();
    dotenv::from_path("4_backend/4_2_http/.env").unwrap();

    let db_url = env::var("DATABASE_URL").unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .unwrap();

    let result = sqlx::query!(
        r#"
            UPDATE "user"
            SET name = $1
            WHERE id = $2
            RETURNING id, name
            "#,
        "new",
        7
    )
    .fetch_one(&pool)
    .await;

    println!("{:?}", result);
}
