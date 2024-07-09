use std::collections::VecDeque;

pub struct DataStream {
    value: i32,
    k: usize,
    counter: usize,
    stream: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DataStream {
    pub fn new(value: i32, k: i32) -> Self {
        DataStream {
            value,
            k: k as usize,
            counter: 0,
            stream: VecDeque::new(),
        }
    }

    pub fn consec(&mut self, num: i32) -> bool {
        if self.stream.len() == self.k {
            self.stream.pop_front();
        }

        if num == self.value {
            self.counter += 1;
        } else {
            self.counter = 0;
        }
        self.stream.push_back(num);
        self.counter >= self.k
    }
}

/*
 value = 4, k = 3
* stream -> [], counter = 0
* add 4
* stream -> [4], counter = 1
* add 4
* stream -> [4,4], counter = 2
* add 4
* stream -> [4,4,4], counter = 3 [return true]
* add 5
* stream -> [4,4,5], counter = 1 [return false]
*/
