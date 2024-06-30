use std::{collections::HashMap, hash::Hash};

use crate::Solution;

#[derive(PartialEq, Eq)]
struct VecKey(Vec<i32>);

impl Hash for VecKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in &self.0 {
            i.hash(state);
        }
    }
}

//  TODO: Implement this solution using a Trie
impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut map = HashMap::new();
        for row in grid.iter() {
            *map.entry(VecKey(row.clone())).or_insert(0) += 1;
        }

        //  collect the columns
        let mut count = 0;
        for col_index in 0..grid[0].len() {
            let mut column = Vec::new();
            for row in &grid {
                column.push(row[col_index])
            }
            if let Some(exists) = map.get(&VecKey(column)) {
                count += exists;
            }
        }
        count
    }
}
