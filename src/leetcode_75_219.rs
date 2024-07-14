use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        if k == 0 {
            return false;
        }

        let mut counter: HashMap<i32, usize> = HashMap::new();
        let mut left: usize = 0;

        for right in 0..nums.len() {
            if let Some(&prev_index) = counter.get(&nums[right]) {
                if right - prev_index <= k as usize {
                    return true;
                }
            }
            counter.insert(nums[right], right);

            if right - left >= k as usize {
                counter.remove(&nums[left]);
                left += 1;
            }
        }
        false
    }
}

/*
Sample execution

nums = [1,2,3,1], k = 3
left = 0, right = 2, counter = { 1: 0, 2: 1, 3: 1 }
iteration 1 -> right = 3


nums = [1,2,3,1,2,3], k = 2
left = 0, right = 2, counter = { 1: 0, 2: 1 }
iteration 1 -> right = 2, left = 1
 */
