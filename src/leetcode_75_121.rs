use crate::Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let mut min_price = i32::MAX;
        let mut max_diff = 0;

        while right < prices.len() {
            if prices[right] < min_price {
                left = right;
                min_price = prices[right];
            }

            let diff = prices[right] - prices[left];
            max_diff = std::cmp::max(diff, max_diff);

            right += 1;
        }
        max_diff
    }
}

/*
{7} -> diff = 0
{1} -> diff = 0
{1, 5} -> diff = 4
{1, 5, 3} -> diff = 4
{1, 5, 3, 6} -> diff = 5
{1, 5, 3, 6, 4} -> diff = 5

{7} -> diff = 0
{6} -> diff = 0
*/

// impl Solution {
//     pub fn max_profit(prices: Vec<i32>) -> i32 {
//         let mut stack: Vec<i32> = Vec::new();
//         let mut max_diff = 0;
//         let mut min_price = i32::MAX;
//         for price in prices {
//             if stack.is_empty() {
//                 stack.push(price);
//                 min_price = std::cmp::min(price, min_price);
//                 continue;
//             }
//             while let Some(&top) = stack.last() {
//                 if price < top {
//                     stack.pop();
//                 } else {
//                     break;
//                 }
//             }
//             if let Some(&top) = stack.last() {
//                 max_diff = std::cmp::max(max_diff, price - min_price);
//             }
//             min_price = std::cmp::min(price, min_price);
//             stack.push(price);
//         }
//         max_diff
//     }
// }

/*
{7}
{1}
{1, 5} diff = 4
{1, 3} diff = 4
{1, 6} diff = 5
*/

/*
{7} diff = 0
{6} diff = 0
{4} diff = 0
*/

// impl Solution {
//     pub fn max_profit(prices: Vec<i32>) -> i32 {
//         fn dp(
//             i: usize,
//             holding: bool,
//             prices: &Vec<i32>,
//             sold: bool,
//             memo: &mut HashMap<(usize, bool), i32>,
//         ) -> i32 {
//             if sold == true {
//                 return 0;
//             }
//             if i >= prices.len() {
//                 return 0;
//             }
//             if let Some(&cached) = memo.get(&(i, holding)) {
//                 return cached;
//             }
//             let result = {
//                 match holding {
//                     true => {
//                         let sell = dp(i + 1, false, prices, true, memo) + prices[i];
//                         let hold = dp(i + 1, true, prices, sold, memo);
//                         std::cmp::max(sell, hold)
//                     }
//                     false => {
//                         let buy = dp(i + 1, true, prices, sold, memo) - prices[i];
//                         let hold = dp(i + 1, false, prices, sold, memo);
//                         std::cmp::max(buy, hold)
//                     }
//                 }
//             };
//             memo.insert((i, holding), result);
//             result
//         }
//         let mut memo: HashMap<(usize, bool), i32> = HashMap::new();
//         dp(0, false, &prices, false, &mut memo)
//     }
// }
