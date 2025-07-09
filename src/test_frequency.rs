use console::Term;

use crate::frequency::FrequencyMap;

pub async fn test_frequency(language: &str, word: &str, term: &Term) {
    term.write_line(&format!(
        "🔍 Testing frequency for '{}' in language '{}'",
        word, language
    ))
    .unwrap();

    match FrequencyMap::new(language, &term).await.unwrap() {
        Some(freq_map) => {
            match freq_map.get_frequency(word) {
                Some(rank) => {
                    term.write_line(&format!("✅ Word '{}' has frequency rank: {}", word, rank))
                        .unwrap();

                    // Convert rank to approximate proficiency level
                    let level = match rank {
                        1..=1000 => "A1",
                        1001..=2000 => "A2",
                        2001..=3000 => "B1",
                        3001..=5000 => "B2",
                        5001..=8000 => "C1",
                        _ => "C2+",
                    };
                    term.write_line(&format!("📊 Approximate proficiency level: {}", level))
                        .unwrap();
                }
                None => {
                    term.write_line(&format!("❌ Word '{}' not found in frequency data", word))
                        .unwrap();
                }
            }
        }
        None => {
            term.write_line(&format!(
                "❌ No frequency data available for language '{}'",
                language
            ))
            .unwrap();
        }
    }

    return;
}
