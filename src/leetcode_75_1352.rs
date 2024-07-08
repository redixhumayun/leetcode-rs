pub struct ProductOfNumbers {
    prefix: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ProductOfNumbers {
    pub fn new() -> Self {
        Self { prefix: Vec::new() }
    }

    pub fn add(&mut self, num: i32) {
        if num == 0 {
            self.prefix.clear();
            return;
        }
        let result = *self.prefix.last().unwrap_or(&1) * num;
        self.prefix.push(result);
    }

    pub fn get_product(&self, k: i32) -> i32 {
        let k: usize = k.try_into().unwrap();
        if k > self.prefix.len() {
            return 0;
        }
        if self.prefix.len() == k {
            return *self.prefix.last().unwrap();
        }
        *self.prefix.last().unwrap() / self.prefix.get(self.prefix.len() - k - 1).unwrap()
    }
}
