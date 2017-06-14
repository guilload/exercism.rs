use std::collections::VecDeque;


pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|v| encode(*v))
        .collect()
}

fn encode(mut value: u32) -> VecDeque<u8> {
    let last = value as u8 & 0x7f; // set bit #7 to 0
    value >>= 7;

    let mut vd = VecDeque::<u8>::new();
    vd.push_front(last);

    while value != 0 {
        let byte = value as u8 | 0x80; // set bit #7 to 1
        vd.push_front(byte);
        value >>= 7;
    }

    vd
}

pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, ()> {
    let mut start = 0;
    let mut stop = 0;
    let mut values = Vec::<u32>::new();

    while stop < bytes.len() {
        let byte = bytes[stop];
        stop += 1;

        if byte < 0x80 {

            if stop - start > 4 && bytes[start] > 0x8f {
                return Err(());
            }

            values.push(decode(&bytes[start..stop]));
            start = stop;
        }
    }

    if values.is_empty() {
        return Err(());
    }

    Ok(values)
}

fn decode(bytes: &[u8]) -> u32 {
    bytes
        .iter()
        .map(|x| (x & 0x7f) as u32)
        .rev()
        .enumerate()
        .fold(0, |acc, (i, x)| acc | x << 7 * i)
}
