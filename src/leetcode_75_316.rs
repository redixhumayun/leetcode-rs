use std::collections::{HashMap, HashSet};

use crate::Solution;

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut stack: Vec<char> = Vec::new();
        let mut seen: HashSet<char> = HashSet::new();
        let mut last_instance: HashMap<char, usize> = HashMap::new();
        let chars: Vec<char> = s.chars().collect();
        for (index, char) in chars.iter().enumerate() {
            last_instance.insert(*char, index);
        }
        for (index, char) in chars.iter().enumerate() {
            if seen.contains(&char) {
                continue;
            }
            while !stack.is_empty() {
                let last = match stack.last() {
                    Some(l) => l,
                    None => break,
                };

                let last_occurrence = match last_instance.get(last) {
                    Some(l) => l,
                    None => break,
                };

                if char >= last {
                    break;
                }

                if *last_occurrence > index {
                    seen.remove(last);
                    stack.pop();
                    continue;
                }
                break; //  this the last occurrence of this character
            }
            stack.push(*char);
            seen.insert(*char);
        }
        stack.iter().collect()
    }
}
