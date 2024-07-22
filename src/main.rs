pub mod graph;

pub struct Solution {}

fn main() {
    let edges = vec![
        vec![1, 2],
        vec![2, 3],
        vec![2, 4],
        vec![3, 4],
        // vec![3, 5],
        // vec![4, 5],
        // vec![5, 2],
    ];
    let result = Solution::is_cycle_with_union_find(edges);
    println!("result {}", result);
}
