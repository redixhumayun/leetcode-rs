use std::collections::HashSet;

use crate::Solution;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let hash_set_1: HashSet<_> = nums1.into_iter().collect();
        let hash_set_2: HashSet<_> = nums2.into_iter().collect();
        let diff1: Vec<i32> = hash_set_1.difference(&hash_set_2).cloned().collect();
        let diff2: Vec<i32> = hash_set_2.difference(&hash_set_1).cloned().collect();
        return vec![diff1, diff2];
    }
}
