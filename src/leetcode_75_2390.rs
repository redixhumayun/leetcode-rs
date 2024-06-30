use std::collections::VecDeque;

use crate::Solution;

impl Solution {
    pub fn remove_stars(s: String) -> String {
        let deq: VecDeque<char> = VecDeque::from(s.chars().collect::<Vec<char>>());
        let mut output: Vec<char> = Vec::new();
        for element in deq.iter() {
            if *element == '*' {
                output.pop();
                continue;
            }
            output.push(*element);
        }
        output.iter().collect()
    }
}
