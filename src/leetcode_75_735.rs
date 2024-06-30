use crate::Solution;

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<i32> = Vec::new();

        for &asteroid in &asteroids {
            let mut asteroid_destroyed = false;

            while let Some(&top) = stack.last() {
                if top > 0 && asteroid < 0 {
                    // Collision scenario
                    if top.abs() < asteroid.abs() {
                        // Top asteroid is destroyed, pop it and continue checking
                        stack.pop();
                        continue;
                    } else if top.abs() == asteroid.abs() {
                        // Both asteroids are destroyed
                        stack.pop();
                    }
                    // Current asteroid is destroyed or both are, break the loop
                    asteroid_destroyed = true;
                    break;
                }
                // No collision scenario
                break;
            }

            if !asteroid_destroyed {
                stack.push(asteroid);
            }
        }

        stack
    }
}
