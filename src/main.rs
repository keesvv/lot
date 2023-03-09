use std::{
    fmt::Display,
    fs::{self, File},
    io,
    path::PathBuf,
};

use lot::Quote;

use clap::{Parser, Subcommand};
use directories::ProjectDirs;
use rand::seq::IteratorRandom;

#[derive(Subcommand)]
enum Command {
    /// Reload the quote cache
    Reload,
}

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    command: Option<Command>,
}

enum Paths {
    CacheDir,
    Cache,
    QuotesDir,
}

impl Paths {
    fn get_project_dirs() -> ProjectDirs {
        ProjectDirs::from("", "", "lot").unwrap()
    }

    fn to_path_buf(&self) -> PathBuf {
        match self {
            Paths::CacheDir => Self::get_project_dirs().cache_dir().to_path_buf(),
            Paths::Cache => Self::CacheDir.to_path_buf().join("quotes.db"),
            Paths::QuotesDir => Self::get_project_dirs().data_dir().join("quotes"),
        }
    }
}

#[derive(Debug)]
enum ReloadError {
    Serialize(bincode::Error),
    Parse(lot::Error),
    IO(io::Error),
}

impl std::error::Error for ReloadError {}
impl Display for ReloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO(e) => write!(f, "IO error: {}", e),
            Self::Serialize(e) => write!(f, "Serialization error: {}", e),
            Self::Parse(e) => write!(f, "Parse error: {}", e),
        }
    }
}

impl From<bincode::Error> for ReloadError {
    fn from(err: bincode::Error) -> Self {
        Self::Serialize(err)
    }
}

impl From<lot::Error> for ReloadError {
    fn from(err: lot::Error) -> Self {
        Self::Parse(err)
    }
}

impl From<io::Error> for ReloadError {
    fn from(err: io::Error) -> Self {
        Self::IO(err)
    }
}

fn reload_quotes() -> Result<(), ReloadError> {
    fs::create_dir_all(Paths::CacheDir.to_path_buf())?;

    let cache_file = File::create(Paths::Cache.to_path_buf())?;
    let quotes_dir = fs::read_dir(Paths::QuotesDir.to_path_buf())?;

    let mut quotes = Vec::new();

    for file in quotes_dir {
        let file = file?;

        if file.path().extension() != Some("txt".as_ref()) {
            continue;
        }

        let content = fs::read_to_string(file.path())?;

        for quote in content.split_terminator("\n%") {
            quotes.push(Quote::try_from(quote)?);
        }
    }

    bincode::serialize_into(cache_file, &quotes).map_err(ReloadError::Serialize)
}

fn get_quotes() -> Result<Vec<Quote>, ReloadError> {
    let cache_file = File::open(Paths::Cache.to_path_buf())?;

    bincode::deserialize_from(cache_file).map_err(ReloadError::Serialize)
}

fn main() {
    match Args::parse().command {
        Some(Command::Reload) => reload_quotes().unwrap(),
        None => {
            let mut rng = rand::thread_rng();
            println!("{}", get_quotes().unwrap().iter().choose(&mut rng).unwrap())
        }
    };
}
