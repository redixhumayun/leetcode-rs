use crate::Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtrack(
            i: usize,
            current: &mut Vec<i32>,
            nums: &Vec<i32>,
            output: &mut Vec<Vec<i32>>,
        ) {
            if i == nums.len() {
                output.push(current.clone());
                return;
            }
            output.push(current.clone());
            for index in i..nums.len() {
                current.push(nums[index]);
                backtrack(index + 1, current, nums, output);
                current.pop();
            }
        }
        let mut output = Vec::new();
        let mut current = Vec::new();
        backtrack(0, &mut current, &nums, &mut output);
        output
    }
}
