use std::borrow::Cow;
use std::collections::HashMap;
use std::fs;
use std::io::{BufWriter, Cursor};
use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
use clap::Parser;
use config::builder::BuilderState;
use config::{Config, ConfigBuilder, FileFormat};
use futures::future;
use image::codecs::jpeg::JpegEncoder;
use image::{io::Reader as ImageReader, DynamicImage, ImageFormat};
use serde::Deserialize;
use tokio::runtime::Builder;
use tokio::time::Instant;
use tracing::info;
use tracing_subscriber::EnvFilter;
use url::Url;

const DEFAULT_LOG_LEVEL: &str = "info";

const CONFIG_FILE: &str = "3_ecosystem/config.json";
const DEFAULT_OUTPUT_DIR: &str = "3_ecosystem/output";
const DEFAULT_QUALITY: Quality = 75;

type NumberOfThreads = usize;
type Quality = u8;

#[derive(Parser)]
#[command(about)]
struct Cli {
    #[arg(long)]
    images: Vec<String>,

    #[arg(long)]
    max_threads: Option<NumberOfThreads>,

    #[arg(long)]
    output_dir: Option<String>,

    #[arg(long)]
    quality: Option<Quality>,
}

#[derive(Debug, Deserialize)]
struct AppConfig {
    images: Vec<String>,
    max_threads: NumberOfThreads,
    output_dir: PathBuf,
    quality: Quality,
}

trait AddCustomConfigs: Sized {
    fn add_default_config(self) -> Result<Self>;
    fn add_cli_config(self, cli: Cli) -> Result<Self>;
}

impl<St: BuilderState> AddCustomConfigs for ConfigBuilder<St> {
    fn add_default_config(mut self) -> Result<Self> {
        let conf_values = HashMap::from([
            ("max_threads", num_cpus::get().to_string()),
            ("output_dir", DEFAULT_OUTPUT_DIR.to_string()),
            ("quality", DEFAULT_QUALITY.to_string()),
        ]);

        for (key, value) in conf_values {
            self = self.set_default(key, value)?;
        }

        Ok(self)
    }

    fn add_cli_config(mut self, cli: Cli) -> Result<Self> {
        if !cli.images.is_empty() {
            self = self.set_override("images", cli.images)?;
        }

        Ok(self
            .set_override_option("max_threads", cli.max_threads.map(|v| v.to_string()))?
            .set_override_option("output_dir", cli.output_dir)?
            .set_override_option("quality", cli.quality.map(|v| v.to_string()))?)
    }
}

fn build_config() -> Result<AppConfig> {
    let cli = Cli::parse();

    let config = Config::builder()
        .add_default_config()?
        .add_source(config::File::new(CONFIG_FILE, FileFormat::Json).required(false))
        .add_source(
            config::Environment::with_prefix("app")
                .try_parsing(true)
                .list_separator(" ")
                .with_list_parse_key("images"),
        )
        .add_cli_config(cli)?
        .build()?;

    Ok(config.try_deserialize::<AppConfig>()?)
}

type ImagePath = String;

#[derive(Debug)]
enum ImageSource {
    LocalFile(ImagePath),
    RemoteUrl(ImagePath),
}

impl ImageSource {
    fn get_image_path(&self) -> &ImagePath {
        match self {
            ImageSource::LocalFile(image_path) => image_path,
            ImageSource::RemoteUrl(image_path) => image_path,
        }
    }
}

fn parse_image_source_by_path(image_path: ImagePath) -> Result<ImageSource> {
    if Path::new(&image_path).is_file() {
        return Ok(ImageSource::LocalFile(image_path));
    }

    if Url::parse(&image_path).is_ok() {
        return Ok(ImageSource::RemoteUrl(image_path));
    }

    bail!("Unknown image source of: {}", image_path);
}

enum Image {
    Jpeg(DynamicImage),
}

async fn get_image_from_source(image_source: &ImageSource) -> Result<Image> {
    let (img, format) = match image_source {
        ImageSource::LocalFile(image_path) => {
            let img_reader = ImageReader::open(image_path)?.with_guessed_format()?;
            let format = img_reader.format();
            let img = img_reader.decode()?;
            (img, format)
        }
        ImageSource::RemoteUrl(image_path) => {
            let response = reqwest::get(image_path).await?;
            let img_reader =
                ImageReader::new(Cursor::new(response.bytes().await?)).with_guessed_format()?;
            let format = img_reader.format();
            let img = img_reader.decode()?;
            (img, format)
        }
    };

    match format {
        Some(format) => match format {
            ImageFormat::Jpeg => Ok(Image::Jpeg(img)),
            _ => {
                bail!("Unsupported image format: {:?}", format);
            }
        },
        None => {
            bail!("Unknown image format of {:?}", image_source);
        }
    }
}

fn get_file_name_by_img_source(img_source: &ImageSource) -> Cow<str> {
    match &img_source {
        ImageSource::LocalFile(img_path) => {
            Cow::Borrowed(Path::new(img_path).file_name().unwrap().to_str().unwrap())
        }
        ImageSource::RemoteUrl(img_path) => Cow::Owned(sanitise_file_name::sanitise(img_path)),
    }
}

fn get_file_extension_from_image(image: &Image) -> &str {
    match &image {
        Image::Jpeg(_) => "jpg",
    }
}

async fn optimize_img(
    img_path: ImagePath,
    quality: Quality,
    output_dir: impl AsRef<Path>,
) -> Result<()> {
    let image_source = parse_image_source_by_path(img_path)?;
    let image = get_image_from_source(&image_source).await?;

    let file_name = get_file_name_by_img_source(&image_source);

    let file_ext = get_file_extension_from_image(&image);

    let output = fs::File::create(
        output_dir
            .as_ref()
            .join(Path::new(file_name.as_ref()).with_extension(file_ext)),
    )?;

    let writer = BufWriter::new(output);

    match image {
        Image::Jpeg(image) => {
            let mut encoder = JpegEncoder::new_with_quality(writer, quality);
            encoder.encode_image(&image)?;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(DEFAULT_LOG_LEVEL)),
        )
        .init();

    let config = build_config()?;

    info!("{:#?}", config);

    fs::create_dir_all(&config.output_dir)?;

    let runtime = Builder::new_multi_thread()
        .worker_threads(config.max_threads)
        .enable_all()
        .build()?;

    let tasks = config.images.into_iter().map(|img| async {
        let start = Instant::now();
        let result = optimize_img(img, config.quality, &config.output_dir).await;
        info!(
            "optimization of image took {:.2} seconds",
            start.elapsed().as_secs_f64()
        );
        result
    });

    runtime.block_on(async { future::try_join_all(tasks).await })?;

    Ok(())
}
