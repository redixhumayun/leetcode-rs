use crate::Solution;

impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let mut result = 0;
        let mut bit_position = 0;

        while bit_position < 32 {
            let mask = 1 << bit_position;
            let a_bit = a & mask;
            let b_bit = b & mask;
            let c_bit = c & mask;

            if a_bit | b_bit != c_bit {
                if c_bit == 0 {
                    if a_bit > 0 && b_bit > 0 {
                        result += 2; //  need to change both to 0
                    } else {
                        result += 1;
                    }
                } else {
                    result += 1;
                }
            }
            bit_position += 1;
        }
        result
    }
}
