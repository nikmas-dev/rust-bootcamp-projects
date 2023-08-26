use anyhow::{bail, Context, Result};
use clap::Parser;
use config::{Config, ConfigBuilder, ConfigError, FileFormat};
use futures::future;
use image::codecs::jpeg::JpegEncoder;
use image::{io::Reader as ImageReader, DynamicImage, ImageFormat};
use serde::Deserialize;
use std::borrow::Cow;
use std::error::Error;
use std::fs;
use std::io::{BufWriter, Cursor};
use std::path::Path;
use tokio::runtime::Builder;
use url::Url;

const IMAGES_FILE: &str = "3_ecosystem/images.json";
const OUTPUT_DIR: &str = "3_ecosystem/output";

#[derive(Parser)]
#[command(about)]
struct Cli {}

#[derive(Debug, Deserialize)]
struct AppConfig {
    images: Vec<String>,
}

fn build_config() -> Result<AppConfig, ConfigError> {
    let cli = Cli::parse();

    let config = Config::builder()
        .add_source(config::File::new(IMAGES_FILE, FileFormat::Json))
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

async fn optimize_img(img_path: ImagePath) -> Result<()> {
    let image_source = parse_image_source_by_path(img_path)?;
    let image = get_image_from_source(&image_source).await?;

    let file_name = get_file_name_by_img_source(&image_source);

    let file_ext = get_file_extension_from_image(&image);

    let output = fs::File::create(
        Path::new(OUTPUT_DIR).join(Path::new(file_name.as_ref()).with_extension(file_ext)),
    )?;

    let writer = BufWriter::new(output);

    match image {
        Image::Jpeg(image) => {
            let mut encoder = JpegEncoder::new_with_quality(writer, 80);
            encoder.encode_image(&image)?;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let config = build_config()?;

    println!("{:#?}", config);

    fs::create_dir_all(OUTPUT_DIR)?;

    let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()?;

    let tasks = config.images.into_iter().map(optimize_img);

    runtime.block_on(async { future::try_join_all(tasks).await })?;

    Ok(())
}
