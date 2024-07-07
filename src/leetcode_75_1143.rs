use crate::Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let text1: Vec<char> = text1.chars().collect();
        let text2: Vec<char> = text2.chars().collect();
        let mut dp = vec![vec![0; text2.len() + 1]; text1.len() + 1];
        for row in 1..=text1.len() {
            for col in 1..=text2.len() {
                if text1.get(row - 1) == text2.get(col - 1) {
                    dp[row][col] = dp[row - 1][col - 1] + 1;
                    continue;
                }
                dp[row][col] = std::cmp::max(dp[row - 1][col], dp[row][col - 1]);
            }
        }
        dp[text1.len()][text2.len()]
    }
}
