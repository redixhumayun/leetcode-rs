pub struct StockSpanner {
    pub stack: Vec<(i32, usize)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {
    pub fn new() -> Self {
        StockSpanner { stack: Vec::new() }
    }

    pub fn next(&mut self, price: i32) -> i32 {
        //  the day the price came in on
        let mut num_of_days = 0;
        while let Some((prev_price, days)) = self.stack.last() {
            if *prev_price > price {
                break;
            }
            num_of_days += days;
            self.stack.pop();
        }
        num_of_days += 1;
        self.stack.push((price, num_of_days));
        return num_of_days as i32;
    }
}
