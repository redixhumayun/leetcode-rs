// use std::collections::{HashMap, HashSet, VecDeque};

use std::collections::HashSet;

use crate::Solution;

struct UnionFind {
    root: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        let root = (0..size).collect::<Vec<_>>();
        let rank = vec![1; size];
        Self { root, rank }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.root[x] == x {
            return x;
        }
        self.root[x] = self.find(self.root[x]);
        self.root[x]
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

    fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        //  build the DSU
        let mut union_find = UnionFind::new(is_connected.len());
        for (index, node) in is_connected.iter().enumerate() {
            for (inner_index, is_connected) in node.iter().enumerate() {
                if *is_connected == 1 {
                    union_find.union(index, inner_index);
                }
            }
        }

        let mut hash_set: HashSet<usize> = HashSet::new();
        for i in 0..is_connected.len() {
            hash_set.insert(union_find.find(i));
        }
        hash_set.len() as i32
    }
}

// impl Solution {
//     pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
//         //  build the graph
//         let mut adj_list = HashMap::new();
//         for (index, node) in is_connected.iter().enumerate() {
//             for (inner_index, is_connected) in node.iter().enumerate() {
//                 if *is_connected == 1 {
//                     adj_list
//                         .entry(index)
//                         .or_insert(Vec::new())
//                         .push(inner_index)
//                 }
//             }
//         }

//         fn bfs(node: usize, visited: &mut HashSet<usize>, adj_list: &HashMap<usize, Vec<usize>>) {
//             let mut queue = VecDeque::new();
//             queue.push_back(node);

//             while let Some(node) = queue.pop_front() {
//                 if !visited.contains(&node) {
//                     visited.insert(node);
//                     for neighbour in adj_list.get(&node).unwrap() {
//                         if !visited.contains(&neighbour) {
//                             queue.push_back(*neighbour);
//                         }
//                     }
//                 }
//             }
//         }

//         //  start the iteration from each source node in the graph
//         let mut connected_components = 0;
//         let mut visited = HashSet::new();
//         for key in adj_list.keys() {
//             if !visited.contains(key) {
//                 connected_components += 1;
//                 bfs(*key, &mut visited, &adj_list);
//             }
//         }

//         connected_components
//     }
// }
