use std::collections::{BinaryHeap, HashMap};

use crate::Solution;

pub struct HeapWrapper {
    count: usize,
    num: i32,
}

impl Ord for HeapWrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.count.cmp(&other.count) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => self.num.cmp(&other.num),
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        }
    }
}

impl PartialOrd for HeapWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HeapWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.num == other.num
    }
}

impl Eq for HeapWrapper {}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut heap: BinaryHeap<HeapWrapper> = BinaryHeap::new();
        let mut hash_map: HashMap<i32, usize> = HashMap::new();
        for num in nums {
            hash_map.entry(num).and_modify(|e| *e += 1).or_insert(0);
        }
        for (num, count) in hash_map.iter() {
            let wrapper = HeapWrapper {
                count: *count,
                num: *num,
            };
            heap.push(wrapper);
        }

        let mut output = Vec::new();
        let mut count = 0;
        while count < k {
            if let Some(popped_value) = heap.pop() {
                output.push(popped_value.num);
            }
            count += 1;
        }
        output
    }
}
