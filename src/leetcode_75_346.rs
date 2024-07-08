pub struct MovingAverage {
    data: Vec<i32>,
    capacity: usize,
    sum: i32,
    index: usize,
    count: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovingAverage {
    pub fn new(size: i32) -> Self {
        MovingAverage {
            data: vec![0; size as usize],
            capacity: size as usize,
            sum: 0,
            index: 0,
            count: 0,
        }
    }

    pub fn next(&mut self, val: i32) -> f64 {
        self.sum -= self.data[self.index];

        self.data[self.index] = val;
        self.sum += val;

        self.index = (self.index + 1) % self.capacity;

        if self.count < self.capacity {
            self.count += 1;
        }

        self.sum as f64 / self.count as f64
    }
}
