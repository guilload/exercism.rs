use std::cmp;


pub fn decode(msg: &str) -> String {
    let mut builder = String::new();
    let mut count = 0;

    for c in msg.chars() {
        if c.is_digit(10) {
            count = count * 10 + c.to_digit(10).unwrap();
        }
        else {
            let n = cmp::max(1, count) as usize;
            builder.push_str(&c.to_string().repeat(n);
            count = 0;
        }
    }

    builder
}

fn append(builder: &mut String, c: char, count: u32) {
     if count > 1 {
        builder.push_str(&count.to_string());
    }

    builder.push(c);
}

pub fn encode(msg: &str) -> String {
    let mut builder = String::new();
    let mut chars = msg.chars();

    if let Some(head) = chars.next() {
        let mut current = head;
        let mut count = 1;

        for c in chars {
            if c == current {
                count += 1;
            }
            else {
                append(&mut builder, current, count);
                current = c;
                count = 1;
            }
        }
        append(&mut builder, current, count);
    }

    builder
}
