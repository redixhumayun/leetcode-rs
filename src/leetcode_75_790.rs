use crate::Solution;

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        if n == 0 || n == 1 {
            return 1;
        }
        if n == 2 {
            return 2;
        }
        let mut dp = vec![0; (n + 1) as usize];
        dp[0] = 1;
        dp[1] = 1;
        dp[2] = 2;
        for i in 3..dp.len() {
            dp[i] = (dp[i - 1] + dp[i - 2] + 2 * dp[i - 3]) % MOD;
        }
        return dp[n as usize];
    }
}
