use std::collections::{HashMap, HashSet};

pub struct TwoSum {
    hash_map: HashMap<i32, u64>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TwoSum {
    pub fn new() -> Self {
        TwoSum {
            hash_map: HashMap::new(),
        }
    }

    pub fn add(&mut self, number: i32) {
        *self.hash_map.entry(number).or_insert(0) += 1;
    }

    pub fn find(&self, value: i32) -> bool {
        for (num, count) in self.hash_map.iter() {
            let complement = value - *num;
            if complement == *num {
                if *count > 1 {
                    return true;
                }
                return false;
            }
            if self.hash_map.contains_key(&complement) {
                return true;
            }
        }
        return false;
    }
}
