use crate::Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let mut max_window = 0;
        let mut k = 1;
        while right < nums.len() {
            if nums[right] == 0 {
                if k > 0 {
                    k -= 1;
                    let window = right - left; //  1 removed because zero found & deleted
                    max_window = std::cmp::max(window, max_window);
                    right += 1;
                    continue;
                }
                //  another zero found, so shrink window
                while k == 0 {
                    if nums[left] == 0 {
                        k += 1;
                    }
                    left += 1;
                }
                continue;
            }
            //  number found is a 1, calculate max window
            let window = right - left; //  have to remove 1 no matter what, whether a zero was found or not
            max_window = std::cmp::max(window, max_window);
            right += 1;
        }
        max_window as i32
    }
}
