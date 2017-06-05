use std::collections::HashMap;


pub fn word_count(sentence: &str) -> HashMap<String, u32> {
    sentence
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic() || c.is_digit(10) || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .fold(HashMap::new(), |mut acc, word| {
            *acc.entry(word.to_string()).or_insert(0) += 1;
            acc
        })
}
