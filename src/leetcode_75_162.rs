use crate::Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }
        let nums: Vec<i64> = nums.iter().map(|num| *num as i64).collect();
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] > nums[mid + 1] {
                //  this is a falling slope
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left as i32
    }
}
