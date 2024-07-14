// use crate::Solution;

pub struct KthLargest {
    k: i32,
    nums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut nums = nums;
        nums.sort();
        KthLargest { k, nums }
    }

    pub fn add(&mut self, val: i32) -> i32 {
        let insertion_index = self.binary_search(val);
        self.nums.insert(insertion_index, val);
        let top_index = self.nums.len() - self.k as usize;
        self.nums[top_index]
    }

    fn binary_search(&self, val: i32) -> usize {
        let mut left = 0;
        let mut right = self.nums.len();
        while left < right {
            let mid = left + (right - left) / 2;
            let mid_value = self.nums[mid];
            if val <= mid_value {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}
