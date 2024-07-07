// pub mod leetcode_75_1431;
// pub mod leetcode_75_2;
// pub mod leetcode_75_605;
// use leetcode_75_605::Solution;
pub mod leetcode_75_72;

pub struct Solution {}

fn main() {
    let result = Solution::min_distance("horse".to_string(), "ros".to_string());
    println!("The result {:?}", result);
}
