use std::collections::HashMap;

use crate::Solution;

impl Solution {
    fn are_counters_equal(
        s1_counter: &HashMap<char, usize>,
        s2_counter: &HashMap<char, usize>,
    ) -> bool {
        if s1_counter.len() != s2_counter.len() {
            return false;
        }

        for (key, value) in s1_counter.iter() {
            match s2_counter.get(key) {
                Some(s2_value) if value != s2_value => return false,
                Some(_) => (),
                None => return false,
            }
        }
        true
    }

    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let s1_chars: Vec<char> = s1.chars().collect();
        let mut s1_counter: HashMap<char, usize> = HashMap::new();
        for char in &s1_chars {
            s1_counter.entry(*char).and_modify(|e| *e += 1).or_insert(1);
        }

        let s2_chars: Vec<char> = s2.chars().collect();
        let mut s2_counter: HashMap<char, usize> = HashMap::new();
        let mut match_count = 0;

        let max_window_size = s1_chars.len();

        let mut left = 0;
        let mut right = 0;
        while right < s2_chars.len() {
            s2_counter
                .entry(s2_chars[right])
                .and_modify(|e| *e += 1)
                .or_insert(1);
            if s1_counter.get(&s2_chars[right]) == s2_counter.get(&s2_chars[right]) {
                match_count += 1;
            }

            let current_window_size = (right - left) + 1;
            if current_window_size > max_window_size {
                if s1_counter.get(&s2_chars[left]) == s2_counter.get(&s2_chars[left]) {
                    match_count -= 1;
                }
                s2_counter.entry(s2_chars[left]).and_modify(|e| *e -= 1);
                if let Some(left_count) = s2_counter.get(&s2_chars[left]) {
                    if *left_count == 0 {
                        s2_counter.remove(&s2_chars[left]);
                    }
                }
                left += 1;
            }

            //  determine whether frequency counters are equal
            if match_count == s1_counter.len() {
                return true;
            }

            right += 1;
        }
        false
    }
}
