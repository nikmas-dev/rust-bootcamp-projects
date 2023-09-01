use clap::Parser;
use reqwest::Client;
use serde_json::Value;
use tracing::info;
use tracing_subscriber::EnvFilter;

use api::Command;

const SERVER_URL: &str = "http://localhost:3008/execute-command";
const DEFAULT_LOG_LEVEL: &str = "info";

#[derive(Parser)]
#[command(about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(DEFAULT_LOG_LEVEL)),
        )
        .init();

    let cli = Cli::parse();
    let response = Client::new()
        .post(SERVER_URL)
        .json(&cli.command)
        .send()
        .await
        .unwrap();

    let json_response =
        serde_json::to_string_pretty(&response.json::<Value>().await.unwrap()).unwrap();

    info!("{}", json_response);
}
