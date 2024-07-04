use std::collections::{HashMap, HashSet};

use crate::Solution;

impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        //  build the adj matrix for directed & undirected edges
        // let mut directed = HashMap::new();
        let mut directed = HashSet::new();
        let mut undirected = HashMap::new();
        for (_, connection) in connections.iter().enumerate() {
            directed.insert((connection[0], connection[1]));
            undirected
                .entry(connection[0])
                .or_insert(Vec::new())
                .push(connection[1]);
            undirected
                .entry(connection[1])
                .or_insert(Vec::new())
                .push(connection[0]);
        }

        fn dfs(
            node: i32,
            visited: &mut HashSet<i32>,
            count: &mut i32,
            undirected: &HashMap<i32, Vec<i32>>,
            directed: &HashSet<(i32, i32)>,
        ) {
            if visited.contains(&node) {
                return;
            }
            if let Some(neighbours) = undirected.get(&node) {
                for neighbour in neighbours {
                    if !visited.contains(neighbour) {
                        if directed.contains(&(node, *neighbour)) {
                            *count += 1;
                        }
                        visited.insert(node);
                        dfs(*neighbour, visited, count, undirected, directed);
                    }
                }
            }
        }

        let mut visited = HashSet::new();
        let mut count = 0;
        dfs(0, &mut visited, &mut count, &undirected, &directed);
        count
    }
}
