static CHILLOUT: &'static str = "Whoa, chill out!";
static FINE: &'static str = "Fine. Be that way!";
static SURE: &'static str = "Sure.";
static WHATEVER: &'static str = "Whatever.";


pub fn reply(msg: &str) -> &str {
    if msg.ends_with('?') { SURE }
    else if msg.is_empty() { FINE }
    else if !msg.chars().any(|x| x.is_alphabetic() && x.is_lowercase()) { CHILLOUT }
    else { WHATEVER }
}
