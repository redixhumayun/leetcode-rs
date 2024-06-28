use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        let mut counts1: HashMap<char, u64> = HashMap::new();
        let mut counts2: HashMap<char, u64> = HashMap::new();
        for char in word1.chars() {
            *counts1.entry(char).or_insert(0) += 1;
        }
        for char in word2.chars() {
            *counts2.entry(char).or_insert(0) += 1;
        }
        let mut keys1 = counts1.keys().collect::<Vec<&char>>();
        let mut keys2 = counts2.keys().collect::<Vec<&char>>();

        let mut values1 = counts1.values().collect::<Vec<&u64>>();
        let mut values2 = counts2.values().collect::<Vec<&u64>>();

        keys1.sort();
        keys2.sort();
        values1.sort();
        values2.sort();

        keys1 == keys2 && values1 == values2
    }
}
