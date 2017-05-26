const TWO: u64 = 2;


pub fn square(i: u32) -> u64 {
    if i < 1 || i > 64 {
        panic!("Square must be between 1 and 64");
    }

    TWO.pow(i - 1)
}

pub fn total() -> u64 {
    (0..64).fold(0, |acc, i| acc + TWO.pow(i))
}
