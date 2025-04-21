use console::Term;
use odict::Dictionary;
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

use crate::progress::STYLE_DOWNLOAD;

pub trait Downloader {
    fn url(&self) -> String;

    async fn download(&self, term: &Term) -> anyhow::Result<String> {
        let url = self.url();

        // Create .data directory if it doesn't exist
        let data_dir = PathBuf::from(".data");
        if !data_dir.exists() {
            fs::create_dir_all(&data_dir)?;
        }

        // Create a filename based on URL hash
        let mut hasher = Sha256::new();

        hasher.update(url.as_bytes());

        let filename = format!("{:x}", hasher.finalize());
        let file_path = data_dir.join(&filename);

        // If file exists, read and return it
        if file_path.exists() {
            term.write_line(
                format!("✅ Using cached dictionary from {}", file_path.display()).as_str(),
            )?;

            let mut file = File::open(&file_path)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            return Ok(content);
        }

        let mut response = reqwest::get(&url).await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to download file: {}", response.status());
        }

        let total_size = response.content_length().unwrap_or(0);

        term.write_line(format!("⬇️ Downloading the dictionary from {}...", url).as_str())?;

        let pb = indicatif::ProgressBar::new(total_size);

        pb.set_style(STYLE_DOWNLOAD.clone());

        // Download chunks and update progress bar
        let mut downloaded: u64 = 0;
        let mut content = Vec::new();

        while let Some(chunk) = response.chunk().await? {
            content.extend_from_slice(&chunk);
            downloaded = std::cmp::min(downloaded + (chunk.len() as u64), total_size);
            pb.set_position(downloaded);
        }

        pb.finish_and_clear();

        term.clear_line()?;
        term.write_line("✅ Download complete")?;

        // Cache the downloaded content
        let content_str = String::from_utf8(content.clone())?;
        let mut file = File::create(&file_path)?;

        file.write_all(&content)?;

        Ok(content_str)
    }
}

pub trait Extractor<Entry> {
    fn extract(&self, term: &Term, data: &str) -> anyhow::Result<Vec<Entry>>;
}

pub trait Converter<Entry> {
    fn convert(&mut self, term: &Term, data: &Vec<Entry>) -> anyhow::Result<Dictionary>;
}
