use std::collections::BinaryHeap;

pub struct MKAverage {
    heap: BinaryHeap<i32>,
    count: usize,
    m: usize,
    k: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MKAverage {
    pub fn new(m: i32, k: i32) -> Self {
        MKAverage {
            heap: BinaryHeap::new(),
            count: 0,
            m: m as usize,
            k: k as usize,
        }
    }

    pub fn add_element(&mut self, num: i32) {
        self.heap.push(num);
        self.count += 1;
    }

    pub fn calculate_mk_average(&mut self) -> i32 {
        if self.count < self.m {
            return -1;
        }
        let mut counter = 0;
        let mut to_add_back = Vec::new();
        while counter < self.k {
            if let Some(v) = self.heap.pop() {
                to_add_back.push(v);
            }
            counter += 1;
        }
        counter = 0;
        let mut result = Vec::new();
        while counter < self.m - self.k - 1 {
            result.push(self.heap.pop().unwrap());
            counter += 1;
        }
        let ret_value = result.iter().sum::<i32>() / result.len() as i32;
        to_add_back.iter().for_each(|elem| self.heap.push(*elem));
        ret_value
    }
}
