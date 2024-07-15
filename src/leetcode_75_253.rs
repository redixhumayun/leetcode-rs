use std::{cmp::Reverse, collections::BinaryHeap};

use crate::Solution;

impl Solution {
    pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort();
        let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        let mut rooms = 0;

        for interval in intervals {
            match min_heap.peek() {
                Some(Reverse(top)) => {
                    if interval[0] < *top {
                        min_heap.push(Reverse(interval[1]));
                        rooms += 1;
                    } else {
                        min_heap.pop();
                        min_heap.push(Reverse(interval[1]));
                    }
                }
                None => {
                    min_heap.push(Reverse(interval[1]));
                    rooms += 1;
                }
            }
        }
        rooms
    }
}
