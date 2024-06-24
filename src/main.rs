// pub mod leetcode_75_1431;
// pub mod leetcode_75_2;
// pub mod leetcode_75_605;
// use leetcode_75_605::Solution;
pub mod leetcode_75_345;

pub struct Solution {}

fn main() {
    let s = "a.";
    let result = Solution::reverse_vowels(s.to_string());
    println!("{}", result);
}
