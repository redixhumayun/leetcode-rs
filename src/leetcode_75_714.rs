use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut dp: Vec<Vec<i32>> = vec![vec![0; 2]; prices.len()];
        dp[0][0] = 0;
        dp[0][1] = -prices[0];
        for i in 1..prices.len() {
            dp[i][0] = std::cmp::max(dp[i - 1][0], dp[i - 1][1] + prices[i] - fee);
            dp[i][1] = std::cmp::max(dp[i - 1][1], dp[i - 1][0] - prices[i]);
        }
        dp[prices.len() - 1][0]

        // let mut memo: HashMap<(usize, bool), i32> = HashMap::new();
        // fn dp(
        //     i: usize,
        //     holding: bool,
        //     fee: i32,
        //     prices: &Vec<i32>,
        //     memo: &mut HashMap<(usize, bool), i32>,
        // ) -> i32 {
        //     if i >= prices.len() {
        //         return 0;
        //     }
        //     if let Some(&v) = memo.get(&(i, holding)) {
        //         return v;
        //     }

        //     let result = {
        //         if holding {
        //             std::cmp::max(
        //                 dp(i + 1, false, fee, prices, memo) - fee + prices[i],
        //                 dp(i + 1, true, fee, prices, memo),
        //             )
        //         } else {
        //             std::cmp::max(
        //                 dp(i + 1, true, fee, prices, memo) - prices[i],
        //                 dp(i + 1, false, fee, prices, memo),
        //             )
        //         }
        //     };
        //     memo.insert((i, holding), result);
        //     result
        // }
        // dp(0, false, fee, &prices, &mut memo)
    }
}
