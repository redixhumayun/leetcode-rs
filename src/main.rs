// pub mod leetcode_75_1431;
// pub mod leetcode_75_2;
// pub mod leetcode_75_605;
// use leetcode_75_605::Solution;
pub mod leetcode_75_724;

pub struct Solution {}

fn main() {
    let nums = [1, 2, 3];
    let r = Solution::pivot_index(nums.to_vec());
    println!("{}", r);
}
