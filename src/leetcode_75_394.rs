use crate::Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let mut stack_chars: Vec<char> = Vec::new();
        let mut output = "".to_string();

        for char in chars {
            if char == ']' {
                let mut inner_string = "".to_string();
                while let Some(last) = stack_chars.last() {
                    if *last == '[' {
                        stack_chars.pop();
                        break;
                    }
                    let elem = stack_chars.pop().unwrap();
                    inner_string = format!("{}{}", elem, inner_string);
                }
                let mut number = String::new();
                while stack_chars.last().map_or(false, |c| c.is_numeric()) {
                    number.insert(0, stack_chars.pop().unwrap());
                }
                let repeat_count: usize = number.parse().unwrap();
                let repeated_string = inner_string.repeat(repeat_count);
                for c in repeated_string.chars() {
                    stack_chars.push(c);
                }
                continue;
            }
            stack_chars.push(char);
        }

        while let Some(c) = stack_chars.pop() {
            output = format!("{}{}", c, output);
        }
        output
    }
}
