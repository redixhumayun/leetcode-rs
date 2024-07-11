use std::collections::HashSet;

use crate::Solution;

struct UnionFind {
    root: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        let root = (0..size).collect::<Vec<_>>();
        let rank = vec![0; size];
        Self { root, rank }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.root[x] == x {
            return x;
        }
        self.root[x] = self.find(self.root[x]);
        return self.root[x];
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return;
        }

        if self.rank[root_x] > self.rank[root_y] {
            self.root[root_y] = root_x;
        } else if self.rank[root_y] > self.rank[root_x] {
            self.root[root_x] = root_y;
        } else {
            self.root[root_y] = root_x;
            self.rank[root_x] += 1;
        }
    }

    fn get_unique_roots(&mut self) -> Vec<usize> {
        let mut unique_roots = HashSet::new();
        for i in 0..self.root.len() {
            let root = self.find(i);
            unique_roots.insert(root);
        }
        unique_roots.into_iter().collect()
    }

    fn sort_component(&mut self, root: usize, original_string: &str, new_string: &mut Vec<char>) {
        //  get all the nodes in this component
        let mut indices = vec![];
        for i in 0..self.root.len() {
            if self.find(i) == root {
                indices.push(i);
            }
        }

        //  get the associated characters from the string
        let mut chars: Vec<char> = indices
            .iter()
            .map(|index| original_string.chars().nth(*index).unwrap())
            .collect();
        chars.sort();
        for (&index, &char) in indices.iter().zip(chars.iter()) {
            new_string[index] = char;
        }
    }
}

impl Solution {
    pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
        let mut union_find = UnionFind::new(s.len());
        for pair in pairs {
            union_find.union(pair[0] as usize, pair[1] as usize);
        }
        let mut new_string = s.chars().collect();
        let unique_roots = union_find.get_unique_roots();
        for root in unique_roots {
            union_find.sort_component(root, &s, &mut new_string);
        }
        new_string.iter().collect()
    }
}
