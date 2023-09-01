use clap::{Parser};
use reqwest::Client;
use serde_json::Value;

use api::Command;

const SERVER_URL: &str = "http://localhost:3008/execute-command";

#[derive(Parser)]
#[command(about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
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
