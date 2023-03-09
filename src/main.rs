use lot::Quote;

use clap::{Parser, Subcommand};

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

fn main() {
    match Args::parse().command {
        Some(Command::Reload) => todo!("reload"),
        None => println!("{}", Quote::try_from("Lorem ipsum dolor sit amet").unwrap()),
    };
}
