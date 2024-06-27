use crate::Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut area = i32::MIN;
        while left < right {
            let w = (right - left) as i32;
            let h = std::cmp::min(height[left], height[right]);
            area = std::cmp::max(area, w * h);
            if height[left] > height[right] {
                right -= 1;
            } else if height[right] > height[left] {
                left += 1;
            } else {
                //  what do i do if they are equal? does it matter which one i move?
                left += 1;
            }
        }
        area
    }
}
