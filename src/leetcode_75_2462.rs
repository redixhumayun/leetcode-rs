use std::{cmp::Reverse, collections::BinaryHeap};

use crate::Solution;

#[derive(Debug)]
enum Choice {
    Left,
    Right,
    Both,
    Neither,
}

impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        let candidates = candidates as usize;
        let mut left_heap = BinaryHeap::new();
        let mut right_heap = BinaryHeap::new();
        let mut left_range = (0, candidates.min(costs.len()) - 1);
        let mut right_range = ((costs.len() - candidates - 1).min(0), costs.len());

        println!("The left {:?}", &costs[left_range.0..left_range.1]);
        println!("The right {:?}", &costs[right_range.0..right_range.1]);

        for i in 0..candidates.min(costs.len()) {
            left_heap.push(Reverse((costs[i], i)));
        }

        for i in costs.len().saturating_sub(candidates)..costs.len() {
            right_heap.push(Reverse((costs[i], i)));
        }

        let mut cost = 0;
        for _ in 0..k {
            let (left_min, right_min) = (left_heap.peek(), right_heap.peek());

            let choice = match (left_min, right_min) {
                (Some(left), Some(right)) => {
                    let Reverse((left_elem, left_idx)) = left;
                    let Reverse((right_elem, right_idx)) = right;
                    if left_idx == right_idx {
                        Choice::Both
                    } else if left_elem <= right_elem {
                        Choice::Left
                    } else {
                        Choice::Right
                    }
                }
                (Some(_), None) => Choice::Left,
                (None, Some(_)) => Choice::Right,
                (None, None) => Choice::Neither,
            };

            println!("The choice {:?}", choice);
            match choice {
                Choice::Both => {
                    let Reverse((elem, _)) = left_heap.pop().unwrap();
                    right_heap.pop();
                    cost += elem;
                    if left_range.1 + 1 < costs.len() {
                        left_range = (left_range.0, left_range.1 + 1);
                        left_heap.push(Reverse((costs[left_range.1], left_range.1)));
                    }
                    if right_range.1 > 0 {
                        right_range = (right_range.0, right_range.1 - 1);
                        right_heap.push(Reverse((costs[right_range.1], right_range.1)));
                    }
                }
                Choice::Left => {
                    let Reverse((elem, _)) = left_heap.pop().unwrap();
                    cost += elem;
                    if left_range.1 + 1 < costs.len() {
                        left_range = (left_range.0, left_range.1 + 1);
                        left_heap.push(Reverse((costs[left_range.1], left_range.1)));
                    }
                }
                Choice::Right => {
                    let Reverse((elem, _)) = right_heap.pop().unwrap();
                    cost += elem;
                    if right_range.1 > 0 {
                        right_range = (right_range.0, right_range.1 - 1);
                        right_heap.push(Reverse((costs[right_range.1], right_range.1)));
                    }
                }
                Choice::Neither => {
                    break;
                }
            }
        }
        cost as i64
    }
}
