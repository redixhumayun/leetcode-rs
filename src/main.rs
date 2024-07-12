pub mod leetcode_75_1684;

pub struct Solution {}

fn main() {
    let result = Solution::find_redundant_connection(vec![
        vec![1, 2],
        vec![2, 3],
        vec![3, 4],
        vec![1, 4],
        vec![1, 5],
    ]);
    println!("Result {:?}", result);
}
