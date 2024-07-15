use std::{cmp::Reverse, collections::BinaryHeap};

use crate::Solution;

impl Solution {
    pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
        let mut bricks = bricks;
        let mut ladders = ladders;

        let mut heap: BinaryHeap<i32> = BinaryHeap::new();
        for i in 0..heights.len() - 1 {
            let height_diff = heights[i + 1] - heights[i];

            if height_diff <= 0 {
                continue;
            }

            if bricks >= height_diff {
                heap.push(height_diff);
                bricks -= height_diff;
                continue;
            }

            //  no more bricks
            if ladders > 0 {
                if let Some(prev_diff) = heap.pop() {
                    if height_diff < prev_diff {
                        bricks += prev_diff;
                        bricks -= height_diff;
                        heap.push(height_diff);
                    } else {
                        heap.push(prev_diff);
                    }
                }
                ladders -= 1;
                continue;
            }
            return i as i32;
        }
        (heights.len() - 1) as i32
        // let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

        // for i in 0..heights.len() - 1 {
        //     let height_diff = heights[i + 1] - heights[i];

        //     if height_diff > 0 {
        //         heap.push(Reverse(height_diff));
        //         if ladders > 0 {
        //             ladders -= 1;
        //             continue;
        //         }

        //         //  no more ladders left
        //         if let Some(Reverse(prev_diff)) = heap.pop() {
        //             if bricks >= prev_diff {
        //                 bricks -= prev_diff;
        //             } else {
        //                 return i as i32;
        //             }
        //         }
        //     }
        // }
        // return (heights.len() - 1) as i32;
    }
}
