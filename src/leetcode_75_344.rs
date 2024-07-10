use crate::Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let string: String = s.iter().collect();
        fn recurse(i: usize, s: String) -> String {
            if i == s.len() {
                return "".to_string();
            }
            return recurse(i + 1, s.clone())
                + &s.chars()
                    .nth(i)
                    .map_or_else(|| "".to_string(), |c| c.to_string());
        }
        let result = recurse(0, string);
        println!("result {}", result);
    }
}
