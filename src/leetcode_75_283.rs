use crate::Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut left = 0;
        let mut right = 0;
        while right < nums.len() && left < nums.len() {
            if nums[right] == 0 {
                right += 1;
                continue;
            }
            if nums[left] != 0 {
                left += 1;
                continue;
            }
            if nums[right] != 0 && nums[left] == 0 && left < right {
                nums.swap(left, right);
            }
            right += 1;
        }
    }
}
