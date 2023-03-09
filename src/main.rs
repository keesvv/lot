use std::{
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
    Cache,
    QuotesDir,
}

impl Paths {
    fn get_project_dirs() -> ProjectDirs {
        ProjectDirs::from("", "", "lot").unwrap()
    }

    fn to_path_buf(&self) -> PathBuf {
        match self {
            Paths::Cache => Self::get_project_dirs().cache_dir().join("quotes.db"),
            Paths::QuotesDir => Self::get_project_dirs().data_dir().join("quotes"),
        }
    }
}

fn reload_quotes() -> io::Result<()> {
    let cache_file = File::create(Paths::Cache.to_path_buf())?;
    let quotes_dir = fs::read_dir(Paths::QuotesDir.to_path_buf())?;

    // for file in quotes_dir {
    //     let file = file?;
    // }

    todo!("reload");
}

fn main() {
    match Args::parse().command {
        Some(Command::Reload) => reload_quotes().unwrap(),
        None => println!("{}", Quote::try_from("Lorem ipsum dolor sit amet").unwrap()),
    };
}
