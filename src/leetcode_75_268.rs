use crate::Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        for num in &nums {
            result = result ^ num;
        }
        for i in 0..=nums.len() {
            result = result ^ i as i32;
        }
        result
    }
}
