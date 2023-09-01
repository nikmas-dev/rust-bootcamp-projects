use clap::Subcommand;
use serde::{Deserialize, Serialize};

pub type UserName = String;
pub type UserId = i64;
pub type RoleSlug = String;
pub type RoleName = String;
pub type RolePermissions = String;

#[derive(Subcommand, Serialize, Deserialize)]
pub enum Command {
    #[command(subcommand)]
    User(UserCommand),
    #[command(subcommand)]
    Role(RoleCommand),
}

#[derive(Subcommand, Serialize, Deserialize)]
pub enum UserCommand {
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

#[derive(Subcommand, Serialize, Deserialize)]
pub enum RoleCommand {
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
