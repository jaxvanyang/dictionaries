use crate::{traits::Downloader, wiktionary::consts::SUPPORTED_LANGUAGES};

pub struct WiktionaryDownloader {
    pub language: String,
}

impl WiktionaryDownloader {
    pub fn new(language: String) -> Self {
        Self { language }
    }
}

impl Downloader for WiktionaryDownloader {
    fn url(&self) -> String {
        let languages = SUPPORTED_LANGUAGES;
        let language = languages.get(self.language.as_str()).unwrap();

        format!(
            "https://kaikki.org/dictionary/{}/kaikki.org-dictionary-{}.jsonl",
            language, language
        )
    }
}
