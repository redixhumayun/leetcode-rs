// pub mod leetcode_75_1431;
// pub mod leetcode_75_2;
// pub mod leetcode_75_605;
// use leetcode_75_605::Solution;
pub mod leetcode_75_1456;

pub struct Solution {}

fn main() {
    let v = "leetcode";
    let r = Solution::max_vowels(v.to_string(), 3);
    println!("{}", r);
}
