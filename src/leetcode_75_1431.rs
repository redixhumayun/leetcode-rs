pub struct Solution {}

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max = candies.iter().max();
        let mut output = Vec::with_capacity(candies.len());
        if max.is_none() {
            output.fill(true);
            return output;
        }

        let max = *max.unwrap();
        candies.iter().for_each(|amt| {
            let new_amount = amt + extra_candies;
            if new_amount >= max {
                output.push(true);
            } else {
                output.push(false);
            }
        });
        return output;
    }
}
