use std::collections::{hash_map::Entry, HashMap};

pub struct Logger {
    tracker: HashMap<String, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Logger {
    pub fn new() -> Self {
        Logger {
            tracker: HashMap::new(),
        }
    }

    pub fn should_print_message(&mut self, timestamp: i32, message: String) -> bool {
        match self.tracker.entry(message) {
            Entry::Occupied(mut entry) => {
                if timestamp < *entry.get() {
                    false
                } else {
                    entry.insert(timestamp + 10);
                    true
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(timestamp + 10);
                true
            }
        }
    }
}
