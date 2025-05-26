use map_macro::hash_map;
use odict::PartOfSpeech;
use std::{collections::HashMap, sync::LazyLock};

pub const SUPPORTED_LANGUAGES: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    hash_map! {
        "eng" => "English",
        "fra" => "French",
        "ger" => "German",
        "ita" => "Italian",
        "por" => "Portuguese",
        "pol" => "Polish",
        "spa" => "Spanish",
        "swe" => "Swedish",
        "jpn" => "Japanese",
        "rus" => "Russian",
        "ara" => "Arabic",
        "cmn" => "Chinese"
    }
});

pub const POS_MAP: LazyLock<HashMap<&str, PartOfSpeech>> = LazyLock::new(|| {
    hash_map! {
      "abbrev" => PartOfSpeech::Abv,
      "adv_phrase" => PartOfSpeech::PhrAdj,
      "affix" => PartOfSpeech::Aff,
      "article" => PartOfSpeech::Art,
      "character" => PartOfSpeech::Chr,
      "circumpos" => PartOfSpeech::Un,  // TODO: fix this? ger
      "combining_form" => PartOfSpeech::Un,  // TODO: fix this? eng
      "romanization" => PartOfSpeech::Un,  // TODO: fix this? jap
      "contraction" => PartOfSpeech::Contr,
      "circumfix" => PartOfSpeech::Cf,
      "infix" => PartOfSpeech::Inf,
      "interfix" => PartOfSpeech::Intf,
      "noun" => PartOfSpeech::N,
      "particle" => PartOfSpeech::Part,
      "phrase" => PartOfSpeech::Phr,
      "prefix" => PartOfSpeech::Pref,
      "prep_phrase" => PartOfSpeech::PhrPrep,
      "proverb" => PartOfSpeech::Prov,
      "punct" => PartOfSpeech::Punc,
      "suffix" => PartOfSpeech::Suff,
      "symbol" => PartOfSpeech::Sym,
      "verb" => PartOfSpeech::V
    }
});
