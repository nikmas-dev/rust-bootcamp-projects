use clap::{Parser, Subcommand};
use serde::Serialize;

const SERVER_URL: &str = "http://localhost:3008";

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

mod user {
    use reqwest::Client;
    use serde_json::{json, Value};

    use crate::{RoleSlug, UserId, UserName, SERVER_URL};

    pub async fn create(name: UserName) {
        let request_url = format!("{}/api/users", SERVER_URL);
        let response = Client::new()
            .post(&request_url)
            .json(&json!({
                "name": name
            }))
            .send()
            .await
            .unwrap();

        println!("{:?}", response);
        println!(
            "{}",
            serde_json::to_string_pretty(&response.json::<Value>().await.unwrap()).unwrap()
        );
    }

    pub async fn update_name(user_id: UserId, new_name: UserName) {
        let request_url = format!("{}/api/users/{}/name", SERVER_URL, user_id);
        let response = Client::new()
            .put(&request_url)
            .json(&json!({
                "new_name": new_name
            }))
            .send()
            .await
            .unwrap();

        println!("{:?}", response);
        println!(
            "{}",
            serde_json::to_string_pretty(&response.json::<Value>().await.unwrap()).unwrap()
        );
    }

    pub async fn delete(user_id: UserId) {
        let request_url = format!("{}/api/users/{}", SERVER_URL, user_id);
        let response = Client::new().delete(&request_url).send().await.unwrap();

        println!("{:?}", response);
        println!(
            "{}",
            serde_json::to_string_pretty(&response.json::<Value>().await.unwrap()).unwrap()
        );
    }

    pub async fn get_by_id(user_id: UserId) {
        let request_url = format!("{}/api/users/{}", SERVER_URL, user_id);
        let response = Client::new().get(&request_url).send().await.unwrap();

        println!("{:?}", response);
        println!(
            "{}",
            serde_json::to_string_pretty(&response.json::<Value>().await.unwrap()).unwrap()
        );
    }

    pub async fn get_all() {
        let request_url = format!("{}/api/users", SERVER_URL);
        let response = Client::new().get(&request_url).send().await.unwrap();

        println!("{:?}", response);
        println!(
            "{}",
            serde_json::to_string_pretty(&response.json::<Value>().await.unwrap()).unwrap()
        );
    }

    pub async fn add_role_to_user(user_id: UserId, role_slug: RoleSlug) {
        let request_url = format!("{}/api/users/{}/roles/{}", SERVER_URL, user_id, role_slug);
        let response = Client::new().post(&request_url).send().await.unwrap();

        println!("{:?}", response);
        println!(
            "{}",
            serde_json::to_string_pretty(&response.json::<Value>().await.unwrap()).unwrap()
        );
    }

    pub async fn remove_role_from_user(user_id: UserId, role_slug: RoleSlug) {
        let request_url = format!("{}/api/users/{}/roles/{}", SERVER_URL, user_id, role_slug);
        let response = Client::new().delete(&request_url).send().await.unwrap();

        println!("{:?}", response);
        println!(
            "{}",
            serde_json::to_string_pretty(&response.json::<Value>().await.unwrap()).unwrap()
        );
    }
}

mod role {
    use crate::{RoleName, RolePermissions, RoleSlug, SERVER_URL};
    use reqwest::Client;
    use serde_json::json;

    pub async fn create(slug: RoleSlug, name: RoleName, permissions: RoleSlug) {
        let request_url = format!("{}/api/roles", SERVER_URL);
        let response = Client::new()
            .post(&request_url)
            .json(&json!({
                "slug": slug,
                "name": name,
                "permissions": permissions
            }))
            .send()
            .await
            .unwrap();

        println!("{:?}", response);
    }

    pub async fn update_name(slug: RoleSlug, new_name: RoleName) {
        let request_url = format!("{}/api/roles/{}/name", SERVER_URL, slug);
        let response = Client::new()
            .put(&request_url)
            .json(&json!({
                "new_name": new_name,
            }))
            .send()
            .await
            .unwrap();

        println!("{:?}", response);
    }

    pub async fn update_permissions(slug: RoleSlug, new_permissions: RolePermissions) {
        let request_url = format!("{}/api/roles/{}/permissions", SERVER_URL, slug);
        let response = Client::new()
            .put(&request_url)
            .json(&json!({
                "new_permissions": new_permissions,
            }))
            .send()
            .await
            .unwrap();

        println!("{:?}", response);
    }

    pub async fn delete(slug: RoleSlug) {
        let request_url = format!("{}/api/roles/{}", SERVER_URL, slug);
        let response = Client::new().delete(&request_url).send().await.unwrap();

        println!("{:?}", response);
    }

    pub async fn get_by_slug(slug: RoleSlug) {
        let request_url = format!("{}/api/roles/{}", SERVER_URL, slug);
        let response = Client::new().get(&request_url).send().await.unwrap();

        println!("{:?}", response);
    }

    pub async fn get_all() {
        let request_url = format!("{}/api/roles", SERVER_URL);
        let response = Client::new().get(&request_url).send().await.unwrap();

        println!("{:?}", response);
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::User(user_command) => match user_command {
            UserCommand::Create { name } => {
                user::create(name).await;
            }
            UserCommand::UpdateName { id, new_name } => {
                user::update_name(id, new_name).await;
            }
            UserCommand::Delete { id } => {
                user::delete(id).await;
            }
            UserCommand::GetById { id } => {
                user::get_by_id(id).await;
            }
            UserCommand::GetAll => {
                user::get_all().await;
            }
            UserCommand::AddRole { user_id, role_slug } => {
                user::add_role_to_user(user_id, role_slug).await;
            }
            UserCommand::RemoveRole { user_id, role_slug } => {
                user::remove_role_from_user(user_id, role_slug).await;
            }
        },
        Command::Role(role_command) => match role_command {
            RoleCommand::Create {
                slug,
                name,
                permissions,
            } => {
                role::create(slug, name, permissions).await;
            }
            RoleCommand::UpdateName { slug, new_name } => {
                role::update_name(slug, new_name).await;
            }
            RoleCommand::UpdatePermissions {
                slug,
                new_permissions,
            } => {
                role::update_permissions(slug, new_permissions).await;
            }
            RoleCommand::Delete { slug } => {
                role::delete(slug).await;
            }
            RoleCommand::GetBySlug { slug } => {
                role::get_by_slug(slug).await;
            }
            RoleCommand::GetAll => {}
        },
    }
}
