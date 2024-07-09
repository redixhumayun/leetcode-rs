use leetcode_75_2034::StockPrice;

// pub mod leetcode_75_1431;
// pub mod leetcode_75_2;
// pub mod leetcode_75_605;
// use leetcode_75_605::Solution;
pub mod leetcode_75_2034;

pub struct Solution {}

fn main() {
    let mut result = Vec::new();
    let mut stock_price = StockPrice::new();
    stock_price.update(1, 10);
    stock_price.update(2, 5);
    result.push(stock_price.current());
    result.push(stock_price.maximum());
    stock_price.update(1, 3);
    result.push(stock_price.maximum());
    stock_price.update(4, 2);
    result.push(stock_price.minimum());

    println!("{:?}", result);
}
