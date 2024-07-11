pub mod leetcode_75_310;

pub struct Solution {}

fn main() {
    let result = Solution::find_min_height_trees(
        6,
        // 4,
        // vec![vec![1, 0], vec![1, 2], vec![1, 3]],
        vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]],
    );
    println!("Result {:?}", result);
}
