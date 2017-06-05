#![feature(ascii_ctype)]
use std::ascii::AsciiExt;


fn translate(c: char) -> char {
    if c.is_digit(10) { c } else { (219 - c as u8) as char }
}


pub fn decode(message: &str) -> String {
    message
        .to_lowercase()
        .chars()
        .filter(|&c| c.is_alphanumeric())
        .map(translate)
        .collect::<String>()
}

pub fn encode(message: &str) -> String {
    message
        .to_lowercase()
        .chars()
        .filter(|&c| c.is_ascii_alphanumeric())
        .map(translate)
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|group| group.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

