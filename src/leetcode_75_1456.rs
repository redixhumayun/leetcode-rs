use std::collections::HashSet;

use crate::Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let s_chars: Vec<char> = s.chars().collect();
        let vowels = HashSet::from(['a', 'e', 'i', 'o', 'u']);
        let k_usize = k as usize;
        let mut left = 0 as usize;
        let mut right = k_usize - 1;
        let mut count = 0;

        // Initial count of vowels in the first window
        for i in 0..k_usize {
            if vowels.contains(&s_chars[i]) {
                count += 1;
            }
        }
        let mut max_count = count;

        //  slide the window
        while right < s.len() - 1 {
            if vowels.contains(&s_chars[left]) {
                count -= 1;
            }
            if vowels.contains(&s_chars[right + 1]) {
                count += 1;
            }
            left += 1;
            right += 1;
            max_count = std::cmp::max(max_count, count);
        }
        max_count
    }
}
