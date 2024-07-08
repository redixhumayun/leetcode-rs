use leetcode_75_346::MovingAverage;

// pub mod leetcode_75_1431;
// pub mod leetcode_75_2;
// pub mod leetcode_75_605;
// use leetcode_75_605::Solution;
pub mod leetcode_75_346;

pub struct Solution {}

fn main() {
    let mut ma = MovingAverage::new(3);
    let result = ma.next(1);
    let result = ma.next(10);
    let result = ma.next(30);
    println!("{:?}", result);
}
