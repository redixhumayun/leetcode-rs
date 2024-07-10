pub mod leetcode_75_547;

pub struct Solution {}

fn main() {
    let result = Solution::find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]]);
    println!("Result {:?}", result);
}
