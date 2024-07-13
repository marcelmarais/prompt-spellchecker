use lazy_static::lazy_static;
use ngrammatic::{Corpus, CorpusBuilder};
use std::collections::HashSet;

use crate::utils::{is_email, is_url};

const DICTIONARY: &str = include_str!("../assets/english.txt");

fn create_corpus() -> Corpus {
    let mut corpus = CorpusBuilder::new().finish();
    for word in DICTIONARY.lines() {
        corpus.add_text(&word.to_lowercase());
    }
    corpus
}

fn create_word_list() -> HashSet<String> {
    DICTIONARY.lines().map(|s| s.to_lowercase()).collect()
}

lazy_static! {
    static ref CORPUS: Corpus = create_corpus();
    static ref WORD_LIST: HashSet<String> = create_word_list();
}

pub struct SpellChecker;

impl SpellChecker {
    pub fn new() -> Self {
        SpellChecker
    }

    pub fn check(&self, word: &str) -> bool {
        if word.parse::<f64>().is_ok() {
            return true;
        }

        let chars = word.chars();

        if chars.map(|x| x.is_uppercase()).all(|x| x) {
            return true;
        }

        if is_url(word) {
            return true;
        }
        if is_email(word) {
            return true;
        }
        match &word.chars().nth(0) {
            Some(c) => {
                if c.is_uppercase() {
                    return true;
                }
            }
            None => return true,
        }

        WORD_LIST.contains(&word.to_lowercase())
    }

    pub fn suggest(&self, word: &str) -> Option<String> {
        CORPUS
            .search(&word.to_lowercase(), 0.5)
            .first()
            .map(|m| m.text.clone())
    }
}
