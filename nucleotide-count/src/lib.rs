#![feature(plugin)]
#![plugin(phf_macros)]

use std::collections::HashMap;

extern crate phf;


static NUCLEOTIDES: phf::Set<char> = phf_set! {
    'A',
    'C',
    'G',
    'T',
};


pub fn count(nucleotide: char, dna: &str) -> Result<usize, ()> {
    if !NUCLEOTIDES.contains(&nucleotide) || !dna.chars().all(|x| NUCLEOTIDES.contains(&x)) {
        Err(())
    }
    else {
        Ok(dna.chars().filter(|x| *x == nucleotide).count())
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, ()> {
    let mut counts: HashMap<char, usize> = HashMap::new();

    for &nucleotide in NUCLEOTIDES.iter() {
        counts.insert(nucleotide, count(nucleotide, dna)?);
    }

    Ok(counts)
}
