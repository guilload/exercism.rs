use std::collections::HashSet;


pub fn is_pangram(sentence: &str) -> bool {
    sentence
        .to_uppercase()
        .chars()
        .filter(|c| *c >= 'A' && *c <= 'Z')
        .collect::<HashSet<_>>()
        .len() == 26
}
