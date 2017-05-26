static ERR: &'static str = "String lengths don't match.";


pub fn hamming_distance(left: &str, right: &str) -> Result<u32, &'static str> {
    if left.len() != right.len() {
        Err(ERR)
    }
    else {
        Ok(left.chars().zip(right.chars()).filter(|&(l, r)| l != r).count() as u32)
    }
}
