use std::collections::VecDeque;

pub struct RecentCounter {
    counter: u32,
    queue: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {
    pub fn new() -> Self {
        Self {
            counter: 0,
            queue: VecDeque::new(),
        }
    }

    pub fn ping(&mut self, t: i32) -> i32 {
        if self.queue.is_empty() {
            self.queue.push_back(t);
            self.counter += 1;
            return self.counter as i32;
        }
        while let Some(first) = self.queue.front() {
            if first < &(t - 3000) {
                self.queue.pop_front();
                self.counter -= 1;
            } else {
                break;
            }
        }
        self.queue.push_back(t);
        self.counter += 1;
        self.counter as i32
    }
}
