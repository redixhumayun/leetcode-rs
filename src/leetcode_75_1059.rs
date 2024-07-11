use std::collections::{HashMap, HashSet};

use crate::Solution;

type AdjList = HashMap<i32, Vec<i32>>;
type VisitedSet = HashSet<i32>;

impl Solution {
    pub fn leads_to_destination(
        n: i32,
        edges: Vec<Vec<i32>>,
        source: i32,
        destination: i32,
    ) -> bool {
        fn dfs(
            node: i32,
            destination: i32,
            adj_list: &AdjList,
            visiting: &mut VisitedSet,
            visited: &mut VisitedSet,
        ) -> bool {
            if node == destination {
                match adj_list.get(&node) {
                    Some(_) => return false, //  a destination cannot have outgoing edges
                    None => return true,
                }
            }
            if visiting.contains(&node) {
                return false; //  subgraph cycle
            }
            if visited.contains(&node) {
                return true; //  node already visited
            }
            visiting.insert(node);
            if let Some(neighbours) = adj_list.get(&node) {
                for neighbour in neighbours {
                    if !dfs(*neighbour, destination, adj_list, visiting, visited) {
                        return false;
                    }
                }
            } else {
                //  this is not the destination but has no outgoing edges
                return false;
            }
            visiting.remove(&node);
            visited.insert(node);
            true
        }

        // build the adjacency list
        let mut hash_map: AdjList = HashMap::new();
        let mut visiting = HashSet::new();
        let mut visited = HashSet::new();
        for edge in edges {
            hash_map
                .entry(edge[0])
                .and_modify(|e| e.push(edge[1]))
                .or_insert(vec![edge[1]]);
        }
        dfs(source, destination, &hash_map, &mut visiting, &mut visited)
    }
}
