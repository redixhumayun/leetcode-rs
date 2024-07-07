use crate::Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut dp = vec![0; cost.len() + 1];
        dp[0] = cost[0];
        dp[1] = cost[1];
        for i in 2..dp.len() {
            let c = {
                if i == cost.len() {
                    0
                } else {
                    cost[i]
                }
            };
            dp[i] = std::cmp::min(dp[i - 1], dp[i - 2]) + c;
        }
        return dp[cost.len()];
    }
}
