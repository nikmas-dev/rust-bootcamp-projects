use clap::{Parser, Subcommand};
use reqwest::Client;
use serde::Serialize;
use serde_json::Value;

const SERVER_URL: &str = "http://localhost:3008/execute-command";

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

#[derive(Subcommand, Serialize)]
enum Command {
    #[command(subcommand)]
    User(UserCommand),
    #[command(subcommand)]
    Role(RoleCommand),
}

#[derive(Subcommand, Serialize)]
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

#[derive(Subcommand, Serialize)]
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
    let cli = Cli::parse();
    let response = Client::new()
        .post(SERVER_URL)
        .json(&cli.command)
        .send()
        .await
        .unwrap();

    println!(
        "{}",
        serde_json::to_string_pretty(&response.json::<Value>().await.unwrap()).unwrap()
    );
}
