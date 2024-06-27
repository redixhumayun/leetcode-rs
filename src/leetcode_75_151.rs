use crate::Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut vec = Vec::new();
        let mut new_string = "".to_string();
        let mut first_space = true;
        let s = s.trim();
        for char in s.chars() {
            if char == ' ' && !first_space {
                continue;
            }
            if char == ' ' && first_space {
                first_space = false;
                vec.push(new_string.clone());
                new_string = "".to_string();
                continue;
            }
            first_space = true;
            new_string.push(char);
        }
        if !new_string.is_empty() {
            vec.push(new_string);
        }
        vec.reverse();
        vec.join(" ")
    }
}
