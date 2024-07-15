use std::{cmp::Reverse, collections::BinaryHeap};

use crate::Solution;

#[derive(PartialEq, Eq)]
struct HeapWrapper {
    num_of_soldiers: usize,
    index: usize,
}

impl Ord for HeapWrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.num_of_soldiers
            .cmp(&other.num_of_soldiers)
            .then_with(|| self.index.cmp(&other.index))
    }
}

impl PartialOrd for HeapWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut heap: BinaryHeap<Reverse<HeapWrapper>> = BinaryHeap::new();
        for (index, row) in mat.iter().enumerate() {
            let score = row.iter().filter(|elem| **elem == 1).count();
            heap.push(Reverse(HeapWrapper {
                num_of_soldiers: score,
                index,
            }));
        }

        let mut output = Vec::new();
        for _ in 0..k {
            if let Some(Reverse(elem)) = heap.pop() {
                output.push(elem.index as i32);
            }
        }
        output
    }
}

/*
scores = {(2,0),(4,1),(1,2),(2,3),(5,4)}, k = 3
(1,2), (2,0), (2,3)
*/
