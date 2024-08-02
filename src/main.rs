use merge_sort::merge_sort;

pub mod merge_sort;

pub struct Solution {}

fn main() {
    let arr = vec![5, 2, 3, 1, 4];
    let result = merge_sort(arr);
    println!("the result {:?}", result);
}
