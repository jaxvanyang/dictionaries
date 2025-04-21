use std::path::PathBuf;

use clap::{Parser, Subcommand, command};
use console::Term;
use traits::{Converter, Downloader, Extractor};
use utils::save_dictionary;
use wiktionary::{WiktionaryArgs, WiktionaryConverter, WiktionaryDownloader, WiktionaryExtractor};

mod progress;
mod traits;
mod utils;
mod wiktionary;

#[derive(Debug, Parser)]
#[command(name = "odict-convert")]
#[command(about = "Convert other dictionary formats to .odict files", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, help = "Path to save the output dictionary file")]
    output: Option<String>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Wiktionary(WiktionaryArgs),
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let (command_name, language) = match &args.command {
        Commands::Wiktionary(wiktionary_args) => ("wiktionary", wiktionary_args.language.clone()),
    };

    let downloader = match args.command {
        Commands::Wiktionary(args) => WiktionaryDownloader::new(args.language),
    };

    let extractor = match args.command {
        Commands::Wiktionary(_) => WiktionaryExtractor::new(),
    };

    let mut converter = match args.command {
        Commands::Wiktionary(_) => WiktionaryConverter::new(),
    };

    let term = Term::stdout();
    let text = downloader.download(&term).await.unwrap();
    let parsed = extractor.extract(&term, &text).unwrap();
    let dictionary = converter.convert(&term, &parsed).unwrap();

    let output_path: PathBuf = match &args.output {
        Some(path) => path.clone().into(),
        None => format!("out/{}/{}.odict", command_name, language).into(),
    };

    save_dictionary(term, &dictionary, &output_path).unwrap();
}
