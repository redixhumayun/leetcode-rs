use crate::Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }
        let mut nums: Vec<i64> = nums.iter().map(|num| *num as i64).collect();
        nums.insert(0, i64::MIN);
        nums.push(i64::MIN);
        let mut left = 1 as i32;
        let mut right = (nums.len() - 2) as i32;
        while left <= right {
            let mut mid = (left + (right - left) / 2) as usize;
            mid += 1;
            let mid_value = nums[mid];
            if mid_value > nums[(mid - 1) as usize] && mid_value > nums[(mid + 1) as usize] {
                //  found peak
                return (mid - 1) as i32;
            }

            //  not found peak, divide space
            if mid_value < nums[(mid - 1) as usize] {
                right = (mid - 1) as i32;
            } else if mid_value > nums[(mid + 1) as usize] {
                left = (mid + 1) as i32;
            }
        }
        0
    }
}
