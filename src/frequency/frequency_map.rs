use std::{collections::HashMap, path::PathBuf};

use console::Term;
use isolang::Language;

use crate::utils::{decompress_gzip, download_with_progress, hash_url, read_file, write_file};

pub struct FrequencyMap {
    map: HashMap<String, u32>,
}

async fn get_frequency_map(
    language_code: &str,
    source: &str,
    version: &str,
    term: &Term,
) -> anyhow::Result<HashMap<String, u32>> {
    let url = format!(
        "https://object.pouta.csc.fi/OPUS-{}/{}/freq/{}.freq.gz",
        source, version, language_code
    );

    let file_path = PathBuf::from(".data").join(&hash_url(&url));

    let content = match read_file(&file_path)? {
        Some(content) => {
            term.write_line(&format!(
                "✅ Using cached frequency list from {}",
                file_path.display()
            ))?;
            content
        }
        None => {
            term.write_line(format!("⬇️ Downloading frequency list from {}...", url).as_str())?;

            let content = download_with_progress(&url, &file_path).await?;

            term.clear_line()?;
            term.write_line("✅ Download complete")?;

            write_file(&file_path, &content)?;

            content
        }
    };

    let mut map = HashMap::new();

    let decoded = String::from_utf8(decompress_gzip(&content)?)?;

    for line in decoded.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() >= 2 {
            if let Ok(frequency) = parts[0].parse::<u32>() {
                map.insert(parts[1].to_string(), frequency);
            }
        }
    }

    term.write_line(&format!("✅ Loaded frequency data for {} words", map.len()))?;

    Ok(map)
}

fn calc_ranks(map: &HashMap<String, u32>) -> HashMap<String, u32> {
    let mut sorted: Vec<_> = map.iter().collect();

    sorted.sort_by(|a, b| b.1.cmp(a.1));

    let mut ranks = HashMap::new();

    for (rank, (word, _)) in sorted.iter().enumerate() {
        ranks.insert(word.to_string(), rank as u32 + 1);
    }

    ranks
}

impl FrequencyMap {
    pub async fn new(language: &str, term: &Term) -> anyhow::Result<Option<Self>> {
        let source = "OpenSubtitles";
        let version = "v2024";

        if language == "cmn" {
            let traditional = get_frequency_map("zh_TW", source, version, term).await?;

            let simplified = get_frequency_map("zh_CN", source, version, term).await?;

            let frequencies = traditional
                .into_iter()
                .chain(simplified)
                .collect::<HashMap<String, u32>>();

            return Ok(Some(Self {
                map: calc_ranks(&frequencies),
            }));
        }

        if let Some(lang) = Language::from_639_3(language).and_then(|l| l.to_639_1()) {
            let frequencies = get_frequency_map(lang, source, version, term)
                .await
                .unwrap_or_default();

            return Ok(Some(Self {
                map: calc_ranks(&frequencies),
            }));
        }

        term.write_line(&format!(
            "❌ Couldn't find frequency map for language \"{}\"",
            language
        ))?;

        Ok(None)
    }

    pub fn get_frequency(&self, word: &str) -> Option<u32> {
        self.map.get(word).cloned()
    }
}
