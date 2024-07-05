use crate::Solution;

impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut output = Vec::new();
        let mut potions = potions;
        potions.sort();
        for spell in spells {
            let mut left = 0 as i32;
            let mut right = (potions.len() - 1) as i32;
            let value_to_seek = ((success as f64 / spell as f64).ceil()) as i64;
            let mut idx = usize::MAX;
            while left <= right {
                let mid = left + (right - left) / 2;
                let mid_value = potions[mid as usize] as i64;
                if value_to_seek == mid_value {
                    idx = mid as usize;
                    break;
                } else if value_to_seek < mid_value {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
            if idx == usize::MAX {
                output.push(potions.len() as i32 - left);
            } else {
                output.push((potions.len() - idx) as i32);
            }
        }
        output
    }
}
