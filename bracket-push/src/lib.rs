pub struct Brackets {
    text: String,
}

impl Brackets {

    pub fn from(text: &str) -> Self {
        Brackets { text: text.to_string() }
    }

    pub fn are_balanced(self) -> bool {
        let mut stack = String::new();

        for ch in self.text.chars() {
            match ch {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                ')' | ']' | '}' => if stack.pop() != Some(ch) { return false; },
                 _ => (),
            }
        }

        stack.is_empty()
    }

}
