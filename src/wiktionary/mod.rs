use clap::{Args, arg};
use consts::SUPPORTED_LANGUAGES;

mod consts;
mod converter;
mod downloader;
mod extractor;
mod schema;

pub use converter::*;
pub use downloader::*;
pub use extractor::*;

#[derive(Debug, Args)]
pub struct WiktionaryArgs {
    #[arg(value_parser = SUPPORTED_LANGUAGES.keys().collect::<Vec<_>>())]
    pub language: String,
}
