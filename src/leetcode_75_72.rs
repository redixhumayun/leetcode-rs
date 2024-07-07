use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut dp: Vec<Vec<i32>> = vec![vec![0; word2.len() + 1]; word1.len() + 1];
        let word1: Vec<char> = word1.chars().collect();
        let word2: Vec<char> = word2.chars().collect();

        for i in 0..dp.len() {
            dp[i][0] = i as i32;
        }
        for j in 0..dp[0].len() {
            dp[0][j] = j as i32;
        }
        for i in 1..dp.len() {
            for j in 1..dp[i].len() {
                if word1[i - 1] == word2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                    continue;
                }
                dp[i][j] =
                    std::cmp::min(dp[i][j - 1], std::cmp::min(dp[i - 1][j], dp[i - 1][j - 1])) + 1;
            }
        }
        dp[word1.len()][word2.len()]
        // let mut memo: HashMap<(usize, usize), i32> = HashMap::new();
        // fn dp(
        //     i: usize,
        //     j: usize,
        //     word1: &Vec<char>,
        //     word2: &Vec<char>,
        //     memo: &mut HashMap<(usize, usize), i32>,
        // ) -> i32 {
        //     if word1.len() == 0 {
        //         return word2.len() as i32;
        //     }
        //     if i >= word1.len() {
        //         return (word2.len() - j) as i32;
        //     }
        //     if j >= word2.len() {
        //         return (word1.len() - i) as i32;
        //     }
        //     if memo.contains_key(&(i, j)) {
        //         return memo[&(i, j)];
        //     }

        //     let mut result = 0;
        //     if word1[i] == word2[j] {
        //         result += dp(i + 1, j + 1, word1, word2, memo);
        //     } else {
        //         result += std::cmp::min(
        //             dp(i, j + 1, word1, word2, memo), //  insert
        //             std::cmp::min(
        //                 dp(i + 1, j, word1, word2, memo),
        //                 dp(i + 1, j + 1, word1, word2, memo),
        //             ), //  delete and replace
        //         ) + 1;
        //     }
        //     memo.insert((i, j), result);
        //     result
        // }
        // dp(
        //     0,
        //     0,
        //     &word1.chars().collect(),
        //     &word2.chars().collect(),
        //     &mut memo,
        // )
    }
}
