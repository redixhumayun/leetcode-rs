use crate::Solution;

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let mut max_window = 0;
        let mut k_mut = k;
        while right < nums.len() {
            if nums[right] == 0 {
                if k_mut > 0 {
                    //  increase size of window and calculate the new max
                    k_mut -= 1;
                    let window = right - left + 1;
                    max_window = std::cmp::max(window, max_window);
                    right += 1;
                    continue;
                }
                //  k is zero, shrink the window from the left
                while k_mut == 0 {
                    if nums[left] == 0 {
                        k_mut += 1;
                    }
                    left += 1;
                }
                right += 1;
                k_mut -= 1;
                continue;
            }
            //  the number is a 1, increase size of window and calculate new max
            let window = right - left + 1;
            max_window = std::cmp::max(window, max_window);
            right += 1;
        }
        max_window as i32
    }
}
