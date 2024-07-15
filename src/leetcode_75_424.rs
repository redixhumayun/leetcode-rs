use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let mut max_window_size = usize::MIN;
        let mut most_freq_char: usize = 0;
        let k: usize = k.try_into().unwrap();
        let chars: Vec<char> = s.chars().collect();
        let mut hash_map: HashMap<char, usize> = HashMap::new();

        while right < s.len() {
            hash_map
                .entry(chars[right])
                .and_modify(|e| *e += 1)
                .or_insert(1);
            most_freq_char = std::cmp::max(most_freq_char, *hash_map.get(&chars[right]).unwrap());
            let mut window_size = (right - left) + 1;
            while window_size - most_freq_char > k {
                hash_map.entry(chars[left]).and_modify(|e| *e -= 1);
                most_freq_char =
                    std::cmp::max(most_freq_char, *hash_map.get(&chars[left]).unwrap());
                left += 1;
                window_size = (right - left) + 1;
            }
            max_window_size = std::cmp::max(max_window_size, window_size);
            right += 1;
        }
        max_window_size as i32
    }
}

/*
AABABBA, 1
left = 0, right = 0

*/
