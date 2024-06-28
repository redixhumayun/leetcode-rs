use crate::Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut prefix_sum = 0;
        let prefix_left: Vec<i32> = nums
            .iter()
            .map(|num| {
                prefix_sum += num;
                prefix_sum
            })
            .collect();
        prefix_sum = 0;
        let mut prefix_right: Vec<i32> = nums
            .iter()
            .rev()
            .map(|num| {
                prefix_sum += num;
                prefix_sum
            })
            .collect();
        prefix_right.reverse();

        for (index, _) in nums.iter().enumerate() {
            let left = if index == 0 {
                0
            } else {
                prefix_left[index - 1]
            };
            let right = if index == prefix_right.len() - 1 {
                0
            } else {
                prefix_right[index + 1]
            };
            if left == right {
                return index as i32;
            }
        }
        -1
    }
}
