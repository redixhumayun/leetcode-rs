use std::collections::VecDeque;

use crate::Solution;

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut queue: VecDeque<char> = VecDeque::new();
        let mut r_count = 0;
        let mut d_count = 0;
        for char in senate.chars() {
            if char == 'R' {
                r_count += 1;
                queue.push_back(char);
            } else {
                d_count += 1;
                queue.push_back(char);
            }
        }

        let mut prev_senator = ' ';
        while r_count > 0 && d_count > 0 {
            println!("The queue {:?}, prev {}", queue, prev_senator);
            let current_senator = queue.pop_front().unwrap();
            if prev_senator == 'R' && current_senator == 'D' {
                d_count -= 1;
                prev_senator = current_senator;
                continue;
            } else if prev_senator == 'D' && current_senator == 'R' {
                r_count -= 1;
                prev_senator = current_senator;
                continue;
            }
            prev_senator = current_senator;
            queue.push_back(current_senator);
        }
        if r_count == 0 {
            return "Dire".to_string();
        }
        return "Radiant".to_string();
    }
}
