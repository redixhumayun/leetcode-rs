use std::{cmp::Reverse, collections::BinaryHeap};

pub struct MedianFinder {
    max_heap: BinaryHeap<i32>,
    min_heap: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    pub fn new() -> Self {
        MedianFinder {
            max_heap: BinaryHeap::new(),
            min_heap: BinaryHeap::new(),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        match self.max_heap.peek() {
            Some(max_top) => {
                if num <= *max_top {
                    self.max_heap.push(num);
                } else {
                    self.min_heap.push(Reverse(num));
                }
            }
            None => self.max_heap.push(num),
        }

        let max_heap_len = self.max_heap.len();
        let min_heap_len = self.min_heap.len();

        // Use checked subtraction and pattern matching to safely handle potential underflow
        match max_heap_len.checked_sub(min_heap_len) {
            Some(diff) if diff > 1 => {
                // Pop from max heap and push to min heap
                if let Some(left) = self.max_heap.pop() {
                    self.min_heap.push(Reverse(left));
                }
            }
            _ => (),
        }

        match min_heap_len.checked_sub(max_heap_len) {
            Some(diff) if diff > 0 => {
                // Pop from min heap and push to max heap
                if let Some(Reverse(right)) = self.min_heap.pop() {
                    self.max_heap.push(right);
                }
            }
            Some(_) => (),
            None => {
                println!("overflow while doing min_heap_len - max_heap_len")
            }
        }
    }

    pub fn find_median(&self) -> f64 {
        let max_heap_len = self.max_heap.len();
        let min_heap_len = self.min_heap.len();

        if max_heap_len == min_heap_len {
            match (self.max_heap.peek(), self.min_heap.peek()) {
                (Some(left), Some(Reverse(right))) => return (*left as f64 + *right as f64) / 2.0,
                (Some(left), None) => return *left as f64,
                (None, Some(Reverse(_right))) => panic!("unexpected state"),
                (None, None) => return 0.0,
            }
        }

        match self.max_heap.peek() {
            Some(left) => return *left as f64,
            None => return 0.0 as f64,
        }
    }
}
