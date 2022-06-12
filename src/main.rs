use clap::{Parser, Subcommand};
use dimport::{setup, sync};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Init { url: String },
    Sync,
}

fn main() {
    let args = Args::parse();

    println!("{args:?}");
    match args.command {
        Commands::Init { url } => setup(&url).unwrap(),
        Commands::Sync => sync().unwrap(),
    };
}
