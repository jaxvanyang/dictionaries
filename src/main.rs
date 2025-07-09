use std::path::PathBuf;

use self::commands::Commands;
use clap::{Parser, command};
use console::Term;
use processors::{CEDictProcessor, Processor, WiktionaryProcessor};
use utils::save_dictionary;

mod args;
mod commands;
mod frequency;
mod processors;
mod progress;
mod test_frequency;
mod utils;

#[derive(Debug, Parser)]
#[command(name = "odict-convert")]
#[command(about = "Convert other dictionary formats to .odict files", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, help = "Path to save the output dictionary file")]
    output: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let term = Term::stdout();

    match &args.command {
        Commands::TestFrequency { language, word } => {
            test_frequency::test_frequency(language, word, &term).await
        }
        _ => {
            let dictionary = match &args.command {
                Commands::Wiktionary(wiktionary_args) => WiktionaryProcessor::new()
                    .unwrap()
                    .process(&term, Some(wiktionary_args.language.clone()))
                    .await
                    .unwrap(),
                Commands::CEDict => CEDictProcessor::new()
                    .unwrap()
                    .process(&term, Some("cmn".to_string()))
                    .await
                    .unwrap(),
                Commands::TestFrequency { .. } => unreachable!(),
            };

            let (command_name, language) = match &args.command {
                Commands::Wiktionary(wiktionary_args) => {
                    ("wiktionary", wiktionary_args.language.clone())
                }
                Commands::CEDict => ("cedict", "zho-eng".to_string()),
                Commands::TestFrequency { .. } => unreachable!(),
            };

            let output_path: PathBuf = match &args.output {
                Some(path) => path.clone().into(),
                None => format!("out/{}/{}.odict", command_name, language).into(),
            };

            save_dictionary(term, &dictionary, &output_path).unwrap();
        }
    }
}
