pub fn is_valid(s: &str) -> bool {
    let chars: Vec<char> = s.chars().filter(|&c| !c.is_whitespace()).collect();

    chars.iter().all(|c| c.is_digit(10)) &&
    chars.len() > 1  &&

    chars
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, c)| {
            let d = c.to_digit(10).unwrap();
            acc + if i % 2 == 1 { d * 2 - if d > 4 { 9 } else { 0 } } else { d }
        }) % 10 == 0
}
