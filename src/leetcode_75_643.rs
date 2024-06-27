use crate::Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let mut sum: f64 = nums[0..k].iter().map(|&x| x as f64).sum();
        let mut max_avg: f64 = sum / (k as f64);
        for i in k..nums.len() {
            sum = sum - (nums[i - k] as f64);
            sum = sum + (nums[i] as f64);
            let avg = sum / (k as f64);
            max_avg = f64::max(max_avg, avg);
        }
        max_avg
    }
}
