#![feature(step_by)]
use std::collections::HashSet;


pub fn sum_of_multiples(n: u32, multiples: &Vec<u32>) -> u32 {
    let mut candidates = HashSet::new();

    for multiple in multiples {
        for i in (0..n).step_by(*multiple) {
            candidates.insert(i);
        }
    }

    candidates.iter().sum()
}
