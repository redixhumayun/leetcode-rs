use std::collections::VecDeque;

pub struct FrontMiddleBackQueue {
    front: VecDeque<i32>,
    back: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FrontMiddleBackQueue {
    pub fn new() -> Self {
        FrontMiddleBackQueue {
            front: VecDeque::new(),
            back: VecDeque::new(),
        }
    }

    pub fn push_front(&mut self, val: i32) {
        self.front.push_front(val);
    }

    pub fn push_middle(&mut self, val: i32) {
        if self.front.len() < self.back.len() {
            self.front.push_back(val);
        } else {
            self.back.push_front(val);
        }
    }

    pub fn push_back(&mut self, val: i32) {
        self.back.push_back(val);
    }

    pub fn pop_front(&mut self) -> i32 {
        match self.front.pop_front() {
            Some(val) => val,
            None => -1,
        }
    }

    pub fn pop_middle(&mut self) -> i32 {
        if self.front.len() >= self.back.len() {
            match self.front.pop_back() {
                Some(val) => val,
                None => -1,
            }
        } else {
            match self.back.pop_front() {
                Some(val) => val,
                None => -1,
            }
        }
    }

    pub fn pop_back(&mut self) -> i32 {
        if self.back.len() > 0 {
            return match self.back.pop_back() {
                Some(val) => val,
                None => -1,
            };
        }
        return match self.front.pop_back() {
            Some(val) => val,
            None => -1,
        };
    }
}
