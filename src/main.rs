use std::{fs, path::PathBuf};

use lot::{Error, QuoteManager};

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

fn main() -> Result<(), Error> {
    let quotes = QuoteManager::new(&Paths::QuotesDir.to_path_buf(), &Paths::Cache.to_path_buf());

    match Args::parse().command {
        Some(Command::Reload) => {
            fs::create_dir_all(Paths::CacheDir.to_path_buf())?;
            quotes.rebuild_cache()?;
        }
        None => {
            let mut rng = rand::thread_rng();
            println!("{}", quotes.get_cached()?.iter().choose(&mut rng).unwrap())
        }
    };

    Ok(())
}
