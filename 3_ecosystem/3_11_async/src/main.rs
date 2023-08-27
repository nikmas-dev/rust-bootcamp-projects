use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::{fs, io};

use clap::Parser;
use futures::future;
use tokio::runtime::Builder;

const PATH_TO_RESULTING_FILES_DIR: &str = "3_ecosystem/3_11_async/files";

type NumberOfThreads = usize;

#[derive(Parser)]
#[command(about)]
struct Cli {
    #[arg(short, long)]
    file: PathBuf,

    #[arg(short, long)]
    max_threads: Option<NumberOfThreads>,
}

type Link = String;

fn get_links_from_file(file: impl AsRef<Path>) -> io::Result<Vec<Link>> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

async fn download_page_by_link(link: &str) -> Result<(), Box<dyn Error>> {
    let response = reqwest::get(link).await?;
    let content = response.text().await?;

    let file_name = sanitise_file_name::sanitise(link);

    let file_path = Path::new(PATH_TO_RESULTING_FILES_DIR)
        .join(file_name)
        .with_extension("html");

    let mut file = File::create(&file_path)?;
    file.write_all(content.as_bytes())?;

    println!("Downloaded {} to {:?}", link, file_path);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let links = get_links_from_file(cli.file)?;

    fs::create_dir_all(Path::new(PATH_TO_RESULTING_FILES_DIR))?;

    let number_of_threads = cli.max_threads.unwrap_or_else(num_cpus::get);

    let runtime = Builder::new_multi_thread()
        .worker_threads(number_of_threads)
        .enable_all()
        .build()?;

    let tasks = links.iter().map(|link| download_page_by_link(link));

    runtime.block_on(async {
        future::join_all(tasks).await;
    });

    Ok(())
}
