use crate::Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut prod = 1;
        let prefix_left: Vec<i32> = nums
            .iter()
            .map(|num| {
                prod = prod * num;
                prod
            })
            .collect();
        prod = 1;
        let mut prefix_right: Vec<i32> = nums
            .iter()
            .rev()
            .map(|num| {
                prod = prod * num;
                prod
            })
            .collect();
        prefix_right.reverse();
        let mut output = Vec::new();
        for (index, _) in nums.iter().enumerate() {
            let left = if index > 0 {
                prefix_left.get(index - 1).unwrap()
            } else {
                &1
            };
            let right = if index < nums.len() - 1 {
                prefix_right.get(index + 1).unwrap()
            } else {
                &1
            };
            output.push(*left * *right);
        }
        output
    }
}
