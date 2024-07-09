pub struct OrderedStream {
    data: Vec<String>,
    pointer: usize,
    n: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl OrderedStream {
    pub fn new(n: i32) -> Self {
        OrderedStream {
            data: vec!["".to_string(); n as usize],
            pointer: 0,
            n,
        }
    }

    pub fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        if id_key > self.n {
            panic!("the id key cannot be greater than n");
        }
        let id_key: usize = id_key.try_into().unwrap();
        self.data[id_key - 1] = value;
        if self.pointer != id_key - 1 {
            return Vec::new();
        }
        let mut output = Vec::new();
        while self.pointer < self.n as usize && self.data[self.pointer].len() != 0 {
            output.push(self.data[self.pointer].clone());
            self.pointer += 1;
        }
        output
    }
}
