use crate::Solution;

impl Solution {
    fn count_ones(mut n: i32) -> i32 {
        let mut count = 0;
        while n > 0 {
            if n & 1 == 1 {
                count += 1;
            }
            n >>= 1;
        }
        count
    }

    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut result = vec![0; (n + 1) as usize];
        for i in 1..=n as usize {
            result[i] = result[i >> 1] + (i as i32 & 1);
        }
        result
    }
}
