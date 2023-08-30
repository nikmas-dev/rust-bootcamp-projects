use clap::{Parser, Subcommand};

use crate::commands::{role, user};
use crate::models::{Role, RoleName, RolePermissions, RoleSlug, UserData, UserId, UserName};
use crate::repositories::impls::postgres::PostgresRepositoryImpl;

mod commands;
mod models;
mod repositories;

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
        #[arg(long)]
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
        #[arg(long)]
        slug: RoleSlug,
    },
    GetAll,
}

#[tokio::main]
async fn main() {
    dotenv::from_path("4_backend/4_1_db/.env").unwrap();

    let cli = Cli::parse();

    let repo = PostgresRepositoryImpl::new().await.unwrap();

    match cli.command {
        Command::User(user_command) => match user_command {
            UserCommand::Create { name } => {
                let created_user = user::create_user(UserData { name }, &repo).await.unwrap();
                println!("user is successfully created: {:?}", created_user);
            }
            UserCommand::UpdateName { id, new_name } => {
                let updated_user = user::update_user_name(&id, new_name, &repo).await.unwrap();
                println!("user is successfully updated: {:?}", updated_user);
            }
            UserCommand::Delete { id } => {
                let deleted_user = user::delete_user(&id, &repo).await.unwrap();
                println!("user is successfully deleted: {:?}", deleted_user);
            }
            UserCommand::GetById { id } => {
                let user = user::get_user_by_id(&id, &repo).await.unwrap();
                println!("{:?}", user);
            }
            UserCommand::GetAll => {
                let users = user::get_all_users(&repo).await.unwrap();
                println!("{:?}", users);
            }
            UserCommand::AddRole { user_id, role_slug } => {
                user::add_role_to_user(&user_id, &role_slug, &repo)
                    .await
                    .unwrap();
                println!("role is successfully added to user");
            }
            UserCommand::RemoveRole { user_id, role_slug } => {
                user::remove_role_from_user(&user_id, &role_slug, &repo)
                    .await
                    .unwrap();
                println!("role is successfully removed from user");
            }
        },
        Command::Role(role_command) => match role_command {
            RoleCommand::Create {
                slug,
                name,
                permissions,
            } => {
                let created_role = role::create_role(
                    Role {
                        slug,
                        name,
                        permissions,
                    },
                    &repo,
                )
                .await
                .unwrap();
                println!("role is successfully created: {:?}", created_role);
            }
            RoleCommand::UpdateName { slug, new_name } => {
                let updated_role = role::update_role_name(&slug, new_name, &repo)
                    .await
                    .unwrap();
                println!("role name is successfully updated: {:?}", updated_role);
            }
            RoleCommand::UpdatePermissions {
                slug,
                new_permissions,
            } => {
                let updated_role = role::update_role_permissions(&slug, new_permissions, &repo)
                    .await
                    .unwrap();
                println!(
                    "role permissions are successfully updated: {:?}",
                    updated_role
                );
            }
            RoleCommand::Delete { slug } => {
                let deleted_role = role::delete_role(&slug, &repo).await.unwrap();
                println!("role is successfully deleted: {:?}", deleted_role);
            }
            RoleCommand::GetBySlug { slug } => {
                let role = role::get_role_by_slug(&slug, &repo).await.unwrap();
                println!("{:?}", role);
            }
            RoleCommand::GetAll => {
                let roles = role::get_all_roles(&repo).await.unwrap();
                println!("{:?}", roles);
            }
        },
    }
}
