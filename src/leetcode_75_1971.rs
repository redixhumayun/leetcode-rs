use std::collections::{HashMap, HashSet};

use crate::Solution;

type AdjList = HashMap<i32, Vec<i32>>;
type VisitedSet = HashSet<i32>;

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        fn dfs(node: i32, destination: i32, adj_list: &AdjList, visited: &mut VisitedSet) -> bool {
            if node == destination {
                return true;
            }
            if let Some(neighbours) = adj_list.get(&node) {
                for neighbour in neighbours {
                    if visited.contains(&neighbour) {
                        continue;
                    }
                    visited.insert(*neighbour);
                    if dfs(*neighbour, destination, adj_list, visited) {
                        return true;
                    }
                }
            }
            return false;
        }

        let mut adj_list: AdjList = HashMap::new();
        let mut visited: VisitedSet = HashSet::new();
        for edge in edges {
            adj_list
                .entry(edge[0])
                .and_modify(|e| e.push(edge[1]))
                .or_insert(vec![edge[1]]);
            adj_list
                .entry(edge[1])
                .and_modify(|e| e.push(edge[0]))
                .or_insert(vec![edge[0]]);
        }
        visited.insert(source);
        dfs(source, destination, &adj_list, &mut visited)
    }
}
