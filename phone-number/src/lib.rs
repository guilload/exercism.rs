pub fn number(phone_number: &str) -> Option<String> {
    let pn: String = phone_number.chars().filter(|c| c.is_digit(10)).collect();

    Some(pn).and_then(|s| match s.len() {
        11 if s.starts_with('1') => Some(s[1..].to_string()),
        10 => Some(s),
        _  => None,
    })
}

pub fn area_code(phone_number: &str) -> Option<String> {
    number(phone_number).map(|s| s[0..3].to_string())
}

pub fn pretty_print(phone_number: &str) -> String {
    number(phone_number)
        .map(|s| format!("({}) {}-{}", &s[0..3], &s[3..6], &s[6..]))
        .unwrap_or("invalid".to_string())
}
