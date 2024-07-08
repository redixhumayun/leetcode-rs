use std::collections::VecDeque;

pub struct HitCounter {
    stream: VecDeque<(i32, usize)>,
    count: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl HitCounter {
    pub fn new() -> Self {
        HitCounter {
            stream: VecDeque::new(),
            count: 0,
        }
    }

    pub fn hit(&mut self, timestamp: i32) {
        if let Some((last_ts, last_count)) = self.stream.back() {
            let count = last_count.clone();
            if *last_ts == timestamp {
                self.stream.pop_back();
                self.stream.push_back((timestamp, count + 1));
            } else {
                self.stream.push_back((timestamp, 1));
                self.count += 1;
            }
        } else {
            self.stream.push_back((timestamp, 1));
            self.count += 1;
        }
    }

    pub fn get_hits(&mut self, timestamp: i32) -> i32 {
        let start = timestamp - 300;
        while let Some((popped_ts, popped_count)) = self.stream.front() {
            let popped_count = popped_count.clone();
            if *popped_ts > start {
                break;
            }
            self.stream.pop_front();
            self.count -= popped_count;
        }
        self.count as i32
    }
}
