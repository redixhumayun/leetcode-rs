use std::collections::{HashMap, HashSet};

use crate::Solution;

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        type Node = usize;

        struct Graph {
            adj_list: HashMap<Node, Vec<Node>>,
        }

        impl Graph {
            fn new() -> Self {
                Graph {
                    adj_list: HashMap::new(),
                }
            }

            fn add_edge(&mut self, u: Node, v: Node) {
                self.adj_list.entry(u).or_insert(Vec::new()).push(v);
            }

            fn dfs(&self, start: Node, visited: &mut HashSet<Node>) {
                if visited.contains(&start) {
                    return;
                }
                visited.insert(start);
                if let Some(neighbours) = self.adj_list.get(&start) {
                    for &neighbour in neighbours {
                        self.dfs(neighbour, visited);
                    }
                }
            }
        }

        let mut graph = Graph::new();
        for (index, node) in rooms.iter().enumerate() {
            for edge in node {
                graph.add_edge(index, *edge as usize);
            }
        }
        let mut visited = HashSet::new();
        graph.dfs(0, &mut visited);
        visited.len() == rooms.len()
    }
}
