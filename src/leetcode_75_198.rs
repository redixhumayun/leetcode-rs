use crate::Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        // let mut memo: Vec<Option<i32>> = vec![None; nums.len()];
        // fn dp(i: usize, nums: &Vec<i32>, memo: &mut Vec<Option<i32>>) -> i32 {
        //     if i >= nums.len() {
        //         return 0;
        //     }
        //     if let Some(val) = memo[i] {
        //         return val;
        //     }
        //     let result = std::cmp::max(nums[i] + dp(i + 2, nums, memo), dp(i + 1, nums, memo));
        //     memo[i] = Some(result);
        //     result
        // }
        // dp(0, &nums, &mut memo);

        if nums.len() == 0 {
            return 0;
        }
        if nums.len() == 1 {
            return nums[0];
        }
        if nums.len() == 2 {
            return std::cmp::max(nums[0], nums[1]);
        }
        let mut dp: Vec<i32> = vec![0; nums.len()];
        dp[0] = nums[0];
        dp[1] = std::cmp::max(nums[0], nums[1]);
        for i in 2..dp.len() {
            dp[i] = std::cmp::max(nums[i] + dp[i - 2], dp[i - 1]);
        }
        dp[nums.len() - 1]
    }
}
