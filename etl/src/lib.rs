use std::ascii::AsciiExt;
use std::collections::BTreeMap;


pub fn transform(input: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut acc: BTreeMap<char, i32> = BTreeMap::new();

    for (score, letters) in input {
        for letter in letters {
            acc.insert(letter.to_ascii_lowercase(), *score);
        }
    }

    acc
}
