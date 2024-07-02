use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut hash_map: HashMap<i32, i32> = HashMap::new();
        let mut sum = 0;
        let mut count = 0;
        hash_map.insert(0, 1);
        for num in nums {
            sum += num;
            let diff = sum - k;
            if hash_map.contains_key(&diff) {
                count += hash_map.get(&diff).unwrap();
            }
            hash_map.entry(sum).and_modify(|e| *e += 1).or_insert(1);
        }
        count
    }
}
