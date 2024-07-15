use crate::Solution;

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = matrix.len();
        let k = k as usize;

        let mut low = 0;
        let mut high = n * n;
        let rows = matrix.len();
        let cols = matrix[0].len();

        while low < high {
            let mid = low + (high - low) / 2;
            let row = mid / cols;
            let col = mid % cols;
            let mid_value = matrix[row][col];

            let elems_to_left = (row * rows) + col;
            if elems_to_left == k - 1 {
                return mid_value;
            }
            if elems_to_left < k - 1 {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        -1
    }
}
