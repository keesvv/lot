use std::{
    fmt::Display,
    fs::{self, File},
    io,
    path::PathBuf,
};

use lot::Quote;

use clap::{Parser, Subcommand};
use directories::ProjectDirs;

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
    IO(io::Error),
}

impl std::error::Error for ReloadError {}
impl Display for ReloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO(e) => write!(f, "IO error: {}", e),
            Self::Serialize(e) => write!(f, "Serialization error: {}", e),
        }
    }
}

fn reload_quotes() -> Result<(), ReloadError> {
    fs::create_dir_all(Paths::CacheDir.to_path_buf()).map_err(ReloadError::IO)?;

    let cache_file = File::create(Paths::Cache.to_path_buf()).map_err(ReloadError::IO)?;
    // let quotes_dir = fs::read_dir(Paths::QuotesDir.to_path_buf()).map_err(ReloadError::IO)?;

    // TODO: actually collect Vec of quotes
    bincode::serialize_into(
        cache_file,
        &Quote {
            author: None,
            text: String::new(),
        },
    )
    .map_err(ReloadError::Serialize)?;
    Ok(())
}

fn main() {
    match Args::parse().command {
        Some(Command::Reload) => reload_quotes().unwrap(),
        None => println!("{}", Quote::try_from("Lorem ipsum dolor sit amet").unwrap()),
    };
}
