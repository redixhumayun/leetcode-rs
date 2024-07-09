use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Location {
    name: String,
    score: i32,
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score
            .cmp(&other.score)
            .then_with(|| other.name.cmp(&self.name))
    }
}

pub struct SORTracker {
    min_heap: BinaryHeap<Reverse<Location>>,
    max_heap: BinaryHeap<Location>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SORTracker {
    pub fn new() -> Self {
        SORTracker {
            min_heap: BinaryHeap::new(),
            max_heap: BinaryHeap::new(),
        }
    }

    pub fn add(&mut self, name: String, score: i32) {
        let location = Location {
            name: name.clone(),
            score,
        };
        if let Some(Reverse(left_top)) = self.min_heap.peek() {
            if score > left_top.score || score == left_top.score && name < left_top.name {
                let Reverse(left_top) = self.min_heap.pop().unwrap();
                self.min_heap.push(Reverse(location));
                self.max_heap.push(left_top);
                return;
            }
        }

        self.max_heap.push(location);
        println!("max heap {:?}", self.max_heap);
    }

    pub fn get(&mut self) -> String {
        let location = self.max_heap.pop().unwrap();
        self.min_heap.push(Reverse(location.clone()));
        location.name
    }
}
