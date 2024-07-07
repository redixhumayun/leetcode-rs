// pub mod leetcode_75_1431;
// pub mod leetcode_75_2;
// pub mod leetcode_75_605;
// use leetcode_75_605::Solution;
pub mod leetcode_75_1268;

pub struct Solution {}

fn main() {
    let products = vec![
        "mobile".to_string(),
        "mouse".to_string(),
        "moneypot".to_string(),
        "monitor".to_string(),
        "mousepad".to_string(),
    ];
    let result = Solution::suggested_products(products, "mouse".to_string());
    println!("{:?}", result);
}
