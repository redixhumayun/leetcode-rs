use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() <= 1 {
            return true;
        }
        if nums[0] == 0 {
            return false;
        }
        let mut index = (nums.len() - 2) as i32;
        let mut can_jump = true;
        let mut min_jump = 1;
        while index >= 0 {
            if nums[index as usize] < min_jump {
                min_jump += 1;
                can_jump = false;
            } else {
                can_jump = true;
                min_jump = 1;
            }
            index -= 1;
        }
        can_jump
        // fn dp(i: usize, nums: &Vec<i32>, memo: &mut HashMap<usize, bool>) -> bool {
        //     if i >= nums.len() - 1 {
        //         return true;
        //     }
        //     if let Some(result) = memo.get(&i) {
        //         return *result;
        //     }
        //     if nums[i] == 0 {
        //         memo.insert(i, false);
        //         return false;
        //     }
        //     for jump in (1..=nums[i]).rev() {
        //         if dp(i + jump as usize, nums, memo) == true {
        //             memo.insert(i, true);
        //             return true;
        //         }
        //     }
        //     memo.insert(i, false);
        //     return false;
        // }
        // let mut memo: HashMap<usize, bool> = HashMap::new();
        // dp(0, &nums, &mut memo)
    }
}
