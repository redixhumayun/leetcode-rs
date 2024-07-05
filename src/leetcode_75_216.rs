use crate::Solution;

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut output: Vec<Vec<i32>> = Vec::new();
        fn backtrack(curr: &mut Vec<i32>, i: usize, k: i32, n: i32, output: &mut Vec<Vec<i32>>) {
            if curr.len() == k as usize {
                if curr.iter().map(|&x| x).sum::<i32>() == n {
                    output.push(curr.to_vec());
                    return;
                }
                return;
            }
            for number in i..=9 {
                curr.push(number as i32);
                backtrack(curr, number + 1, k, n, output);
                curr.pop();
            }
        }
        backtrack(&mut Vec::new(), 1, k, n, &mut output);
        output
    }
}
