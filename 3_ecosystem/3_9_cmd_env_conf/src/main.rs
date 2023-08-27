use log::Level;
use serde::Deserialize;
use std::net::IpAddr;
use std::time::Duration;
use std::{path::PathBuf, string::String};
use url::Url;

use clap::Parser;

use config::{Config, ConfigError, Environment, File, FileFormat};

const DEFAULT_CONF_PATH: &str = "config.toml";

#[derive(Debug, Deserialize)]
pub struct Settings {
    mode: Mode,
    server: Server,
    db: Db,
    log: Log,
    background: Background,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let cli = Cli::parse();

        let path = cli.conf.unwrap_or(PathBuf::from(DEFAULT_CONF_PATH));
        let path = path.to_str().unwrap();

        let conf = Config::builder()
            .add_source(create_default_conf())
            .add_source(File::new(path, FileFormat::Toml))
            .add_source(Environment::with_prefix("conf"))
            .set_override_option("mode.debug", cli.debug.map(|mode| mode.to_string()))?
            .build()?;

        conf.try_deserialize()
    }
}

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(short, long, value_name = "CONF_FILE")]
    conf: Option<PathBuf>,

    #[arg(short, long)]
    debug: Option<bool>,
}

use std::collections::HashMap;
fn create_default_conf() -> Config {
    let conf_values = HashMap::from([
        ("mode.debug", "false"),
        ("server.external_url", "http://127.0.0.1"),
        ("server.http_port", "8081"),
        ("server.grpc_port", "8082"),
        ("server.healthz_port", "10025"),
        ("server.metrics_port", "9199"),
        ("db.mysql.host", "127.0.0.1"),
        ("db.mysql.port", "3306"),
        ("db.mysql.dating", "default"),
        ("db.mysql.user", "root"),
        ("db.mysql.pass", ""),
        ("db.mysql.connections.max_idle", "30"),
        ("db.mysql.connections.max_open", "30"),
        ("log.app.level", "INFO"),
        ("background.watchdog.period", "5s"),
        ("background.watchdog.limit", "10"),
        ("background.watchdog.lock_timeout", "4s"),
    ]);

    let mut result_conf = Config::builder();

    for (key, value) in conf_values {
        result_conf = result_conf.set_default(key, value).unwrap();
    }

    result_conf.build().unwrap()
}

#[derive(Debug, Deserialize)]
pub struct Mode {
    debug: bool,
}

type Port = u16;

#[derive(Debug, Deserialize)]
pub struct Server {
    external_url: Url,
    http_port: Port,
    grpc_port: Port,
    healthz_port: Port,
    metrics_port: Port,
}

#[derive(Debug, Deserialize)]
pub struct MySql {
    host: IpAddr,
    port: Port,
    dating: String,
    user: String,
    pass: String,
    connections: Connections,
}

#[derive(Debug, Deserialize)]
pub struct Db {
    mysql: MySql,
}

#[derive(Debug, Deserialize)]
pub struct Connections {
    max_idle: u32,
    max_open: u32,
}

#[derive(Debug, Deserialize)]
pub struct App {
    level: Level,
}

#[derive(Debug, Deserialize)]
pub struct Log {
    app: App,
}

#[derive(Debug, Deserialize)]
pub struct Background {
    watchdog: Watchdog,
}

#[derive(Debug, Deserialize)]
pub struct Watchdog {
    #[serde(with = "humantime_serde")]
    period: Duration,
    #[serde(with = "humantime_serde")]
    lock_timeout: Duration,
    limit: u32,
}

fn main() {
    let settings = Settings::new().unwrap();
    println!("{:#?}", settings);
}
