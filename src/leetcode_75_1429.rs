use std::collections::{HashMap, VecDeque};

pub struct FirstUnique {
    queue: VecDeque<i32>,
    hash_map: HashMap<i32, i32>,
}

impl FirstUnique {
    pub fn new(nums: Vec<i32>) -> Self {
        let mut queue: VecDeque<i32> = VecDeque::new();
        let mut hash_map: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            queue.push_back(num);
            *hash_map.entry(num).or_insert(0) += 1;
        }
        FirstUnique { queue, hash_map }
    }

    pub fn show_first_unique(&mut self) -> i32 {
        while let Some(num) = self.queue.front() {
            if let Some(&count) = self.hash_map.get(&num) {
                if count > 1 {
                    self.queue.pop_front();
                } else {
                    return *num;
                }
            }
        }
        -1
    }

    pub fn add(&mut self, value: i32) {
        self.queue.push_back(value);
        *self.hash_map.entry(value).or_insert(0) += 1;
    }
}
