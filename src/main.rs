pub mod leetcode_75_253;

pub struct Solution {}

fn main() {
    let result = Solution::min_meeting_rooms(vec![vec![5, 8], vec![6, 8]]);
    println!("result {:?}", result);
}
