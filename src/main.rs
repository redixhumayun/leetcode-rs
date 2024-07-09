use leetcode_75_2526::DataStream;

// pub mod leetcode_75_1431;
// pub mod leetcode_75_2;
// pub mod leetcode_75_605;
// use leetcode_75_605::Solution;
pub mod leetcode_75_2526;

pub struct Solution {}

fn main() {
    let mut result = Vec::new();
    let mut data_stream = DataStream::new(4, 3);
    result.push(data_stream.consec(4));
    result.push(data_stream.consec(4));
    result.push(data_stream.consec(4));
    result.push(data_stream.consec(3));
    println!("{:?}", result);
}
