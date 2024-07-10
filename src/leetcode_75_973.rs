use std::{cmp::Reverse, collections::BinaryHeap};

use crate::Solution;

#[derive(PartialEq, Eq)]
struct Point {
    distance: i32,
    coordinates: Vec<i32>,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut heap: BinaryHeap<Reverse<Point>> = BinaryHeap::new();
        for point in points {
            let p_x = point[0];
            let p_y = point[1];
            let distance = p_x.pow(2) + p_y.pow(2);
            heap.push(Reverse(Point {
                distance,
                coordinates: point,
            }));
        }

        let mut output = Vec::new();
        for _ in 0..k {
            if let Some(Reverse(point)) = heap.pop() {
                output.push(point.coordinates);
            } else {
                break;
            }
        }
        output
    }
}
