use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        let mut bitmap1 = 0;
        let mut bitmap2 = 0;
        let mut counts1: HashMap<char, u64> = HashMap::new();
        let mut counts2: HashMap<char, u64> = HashMap::new();
        for char in word1.chars() {
            let index = char as u32 - 'a' as u32;
            bitmap1 = bitmap1 | 1 << index;
            *counts1.entry(char).or_insert(0) += 1;
        }
        for char in word2.chars() {
            let index = char as u32 - 'a' as u32;
            bitmap2 = bitmap2 | 1 << index;
            *counts2.entry(char).or_insert(0) += 1;
        }

        if bitmap1 != bitmap2 {
            return false;
        }

        let mut values1 = counts1.values().collect::<Vec<&u64>>();
        let mut values2 = counts2.values().collect::<Vec<&u64>>();
        values1.sort();
        values2.sort();

        values1 == values2
    }
}
