fn translate(c: char, base: u8, key: u8) -> char {
    (base + (c as u8 - base + key) % 26) as char
}

pub fn rotate(message: &str, key: u8) -> String {
    message
        .chars()
        .map(|c| match c {
            'a'...'z' => translate(c, 97, key),
            'A'...'Z' => translate(c, 65, key),
            _ => c,
        })
        .collect::<String>()
}
