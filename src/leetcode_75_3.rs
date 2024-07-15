use std::collections::HashSet;

use crate::Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut left = 0;
        let mut right = 0;
        let mut max_window_size = 0;
        let mut hash_set: HashSet<char> = HashSet::new();
        while right < s.len() {
            let char = chars[right];
            if hash_set.contains(&char) {
                hash_set.remove(&chars[left]);
                left += 1;
                continue;
            }
            hash_set.insert(chars[right]);
            let window_size = (right - left) + 1;
            max_window_size = std::cmp::max(window_size, max_window_size);
            right += 1;
        }
        max_window_size as i32
    }
}

/*
s = "abcabcbb"
(left, right) = (0, 0)

*/
