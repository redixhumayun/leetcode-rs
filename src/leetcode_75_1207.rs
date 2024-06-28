use std::collections::{HashMap, HashSet};

use crate::Solution;

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut hash_map = HashMap::new();
        arr.iter().for_each(|num| {
            match hash_map.get(num) {
                Some(value) => hash_map.insert(num, value + 1),
                None => hash_map.insert(num, 1),
            };
        });
        let values_len = hash_map.values().len();
        let hash_set_v: HashSet<&i32> = hash_map.values().into_iter().collect();
        values_len == hash_set_v.len()
    }
}
