use crate::Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums_sort = nums.clone();
        nums_sort.sort();
        let mut left = 0;
        let mut right = nums_sort.len() - 1;
        let mut counter = 0;
        while left < right {
            let sum = nums_sort[left] + nums_sort[right];
            if sum == k {
                counter += 1;
                left += 1;
                right -= 1;
                continue;
            }
            if sum > k {
                right -= 1;
            } else {
                left += 1;
            }
        }
        counter
    }
}
