pub struct Solution {}

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut flowerbed = flowerbed.clone();
        let mut counter = 0;
        for i in 0..flowerbed.len() {
            if flowerbed[i] == 0 {
                let left = if i == 0 { 0 } else { flowerbed[i - 1] };
                let right = if i == flowerbed.len() - 1 {
                    0
                } else {
                    flowerbed[i + 1]
                };
                if left == 0 && right == 0 {
                    counter += 1;
                    flowerbed[i] = 1;
                }
            }
        }
        counter >= n
    }
}
