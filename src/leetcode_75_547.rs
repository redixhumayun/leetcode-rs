use std::collections::{HashMap, HashSet, VecDeque};

use crate::Solution;

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        //  build the graph
        let mut adj_list = HashMap::new();
        for (index, node) in is_connected.iter().enumerate() {
            for (inner_index, is_connected) in node.iter().enumerate() {
                if *is_connected == 1 {
                    adj_list
                        .entry(index)
                        .or_insert(Vec::new())
                        .push(inner_index)
                }
            }
        }

        fn bfs(node: usize, visited: &mut HashSet<usize>, adj_list: &HashMap<usize, Vec<usize>>) {
            let mut queue = VecDeque::new();
            queue.push_back(node);

            while let Some(node) = queue.pop_front() {
                if !visited.contains(&node) {
                    visited.insert(node);
                    for neighbour in adj_list.get(&node).unwrap() {
                        if !visited.contains(&neighbour) {
                            queue.push_back(*neighbour);
                        }
                    }
                }
            }
        }

        //  start the iteration from each source node in the graph
        let mut connected_components = 0;
        let mut visited = HashSet::new();
        for key in adj_list.keys() {
            if !visited.contains(key) {
                connected_components += 1;
                bfs(*key, &mut visited, &adj_list);
            }
        }

        connected_components
    }
}
