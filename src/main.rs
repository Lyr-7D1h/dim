use clap::{Parser, Subcommand};
use dim::Dim;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Setup { url: String },
    Sync,
}

fn main() {
    let args = Args::parse();

    let dim = Dim::init().unwrap();

    println!("{args:?}");
    match args.command {
        Commands::Setup { url } => dim.setup(url).unwrap(),
        Commands::Sync => dim.sync().unwrap(),
    };
}
