use leetcode_75_1500::FileSharing;

// pub mod leetcode_75_1431;
// pub mod leetcode_75_2;
// pub mod leetcode_75_605;
// use leetcode_75_605::Solution;
pub mod leetcode_75_1500;

pub struct Solution {}

fn main() {
    let mut result = Vec::new();
    let mut file_sharing = FileSharing::new(4);
    result.push(file_sharing.join([1, 2].to_vec()));
    result.push(file_sharing.join([2, 3].to_vec()));
    file_sharing.request(1, 3);
    file_sharing.request(2, 2);

    println!("{:?}", result);
}
