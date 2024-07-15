use std::collections::BinaryHeap;

use crate::Solution;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::new();
        for stone in stones {
            heap.push(stone);
        }
        while heap.len() > 1 {
            let stone_1 = heap.pop().unwrap();
            let stone_2 = heap.pop().unwrap();
            if stone_1 != stone_2 {
                let result = stone_1 - stone_2;
                heap.push(result);
            }
        }
        if let Some(last) = heap.pop() {
            return last;
        }
        0
    }
}
