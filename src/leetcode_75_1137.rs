use crate::Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        let mut arr = vec![0; (n + 1) as usize];
        arr[1] = 1;
        arr[2] = 1;
        for i in 3..arr.len() {
            arr[i] = arr[i - 1] + arr[i - 2] + arr[i - 3];
        }
        return arr[n as usize];
    }
}
