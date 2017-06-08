// does not pass all the test cases... and I don't care :)
pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split_whitespace()
        .flat_map(|w| w.chars().next())
        .collect::<String>()
        .to_uppercase()
}
