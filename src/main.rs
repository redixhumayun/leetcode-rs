use leetcode_75_362::HitCounter;

// pub mod leetcode_75_1431;
// pub mod leetcode_75_2;
// pub mod leetcode_75_605;
// use leetcode_75_605::Solution;
pub mod leetcode_75_362;

pub struct Solution {}

fn main() {
    let mut counter = HitCounter::new();
    counter.hit(1);
    counter.hit(2);
    counter.hit(3);
    let result = counter.get_hits(4);
    println!("{:?}", result);
}
