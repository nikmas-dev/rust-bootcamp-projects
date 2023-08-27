use std::fs::File;
use std::io;
use std::iter::Filter;
use std::path::Path;
use tracing::metadata::LevelFilter;
use tracing::{debug, error, info, subscriber, trace, Level};
use tracing_subscriber::filter::{FilterFn, Targets};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, Layer};

fn main() {
    let access_logger = fmt::layer()
        .json()
        .with_writer(File::create(Path::new("3_ecosystem/3_8_log/access.log")).unwrap())
        .with_filter(Targets::default().with_target("access.log", Level::TRACE));

    let app_logger_stdout = fmt::layer()
        .json()
        .with_writer(io::stdout)
        .with_filter(FilterFn::new(|metadata| {
            metadata.target() == "app.log" && metadata.level() >= &Level::WARN
        }));

    let app_logger_stderr = fmt::layer()
        .json()
        .with_writer(io::stderr)
        .with_filter(FilterFn::new(|metadata| {
            metadata.target() == "app.log" && metadata.level() < &Level::WARN
        }));

    let subscriber = tracing_subscriber::registry()
        .with(access_logger)
        .with(app_logger_stdout)
        .with(app_logger_stderr);

    subscriber::set_global_default(subscriber).unwrap();
}
