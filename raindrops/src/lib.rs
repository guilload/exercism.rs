pub fn raindrops(number: u32) -> String {
    let mut builder = String::new();

    if number % 3 == 0 { builder.push_str("Pling") }
    if number % 5 == 0 { builder.push_str("Plang") }
    if number % 7 == 0 { builder.push_str("Plong") }

    if builder.is_empty() { number.to_string() } else { builder }
}
