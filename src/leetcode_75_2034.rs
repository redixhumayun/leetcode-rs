use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap},
};

pub struct StockPrice {
    prices: BTreeMap<i32, i32>,
    min_heap: BinaryHeap<Reverse<(i32, i32)>>, //  (price, ts)
    max_heap: BinaryHeap<(i32, i32)>,          // (price, ts)
    latest_price: i32,
    latest_ts: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockPrice {
    pub fn new() -> Self {
        StockPrice {
            prices: BTreeMap::new(),
            min_heap: BinaryHeap::new(),
            max_heap: BinaryHeap::new(),
            latest_price: 0,
            latest_ts: 0,
        }
    }

    pub fn update(&mut self, timestamp: i32, price: i32) {
        self.prices.insert(timestamp, price);
        self.max_heap.push((price, timestamp));
        self.min_heap.push(Reverse((price, timestamp)));
        if timestamp > self.latest_ts {
            self.latest_price = price;
            self.latest_ts = timestamp;
        }
    }

    pub fn current(&self) -> i32 {
        self.latest_price
    }

    pub fn maximum(&mut self) -> i32 {
        while let Some((price, ts)) = self.max_heap.peek() {
            match self.prices.get_key_value(ts) {
                Some((map_ts, map_price)) => {
                    if ts == map_ts && price == map_price {
                        return *map_price;
                    } else {
                        self.max_heap.pop();
                    }
                }
                None => {
                    self.max_heap.pop();
                }
            }
        }
        panic!("could not find any value in the max heap");
    }

    pub fn minimum(&mut self) -> i32 {
        while let Some(Reverse((price, ts))) = self.min_heap.peek() {
            match self.prices.get_key_value(ts) {
                Some((map_ts, map_price)) => {
                    if ts == map_ts && price == map_price {
                        return *map_price;
                    } else {
                        self.min_heap.pop();
                    }
                }
                None => {
                    self.min_heap.pop();
                }
            }
        }
        panic!("could not find any value in the min heap")
    }
}

/*
 *  (1, 10) -> ts=1, price=10, min_heap=(10, 1), max_heap=(10, 1)
 *  (2, 5) -> ts=2, price=5, min_heap=(5, 2), max_heap=(10, 1)
 *  (1, 3) -> ts=2, price=5, min_heap=(3, 1), max_heap=(2, 5)
 */
