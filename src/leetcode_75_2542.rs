use std::{cmp::Reverse, collections::BinaryHeap};

use crate::Solution;

impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let mut min_heap = BinaryHeap::new();
        let left = 0;
        let right = k;
        let mut sum = 0;
        for i in 0..k as usize {
            min_heap.push(Reverse(nums2[i]));
            sum += nums1[i];
        }
        let mut return_value = 0;
        0
    }
}
