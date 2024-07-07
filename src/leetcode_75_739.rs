use crate::Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<(i32, usize)> = Vec::new();
        let mut output = vec![0; temperatures.len()];
        stack.push((temperatures[0], 0));
        for i in 1..temperatures.len() {
            while stack.last().is_some() && temperatures[i] > stack.last().unwrap().0 {
                let (_, index) = stack.pop().unwrap();
                output[index] = (i - index) as i32;
            }
            stack.push((temperatures[i], i));
        }
        output
    }
}
