// pub mod leetcode_75_1431;
// pub mod leetcode_75_2;
// pub mod leetcode_75_605;
// use leetcode_75_605::Solution;
pub mod leetcode_75_1657;

pub struct Solution {}

fn main() {
    let w1 = "uau";
    let w2 = "ssx";
    let r = Solution::close_strings(w1.to_string(), w2.to_string());
    println!("{:?}", r);
}
