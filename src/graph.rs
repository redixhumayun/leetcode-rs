use std::collections::{hash_map, HashMap, HashSet};

use crate::Solution;

type AdjList = HashMap<i32, Vec<i32>>;

impl Solution {
    pub fn is_dir_graph_cyclic(edges: Vec<Vec<i32>>) -> bool {
        fn is_cycle(
            node: i32,
            adj_list: &AdjList,
            visiting: &mut HashSet<i32>,
            visited: &mut HashSet<i32>,
        ) -> bool {
            //  add the node to the visiting set
            if visited.contains(&node) {
                return false; //  this node has already been visited, so no cycle
            }
            if visiting.contains(&node) {
                return true; //  node re-visited in subgraph
            }
            visiting.insert(node);
            if let Some(neighbours) = adj_list.get(&node) {
                for neighbour in neighbours {
                    if is_cycle(*neighbour, adj_list, visiting, visited) {
                        return true;
                    }
                }
            }
            visiting.remove(&node);
            visited.insert(node);
            false
        }

        let mut adj_list: AdjList = HashMap::new();
        let mut min_edge = i32::MAX;
        let mut max_edge = i32::MIN;
        for edge in &edges {
            min_edge = std::cmp::min(min_edge, std::cmp::min(edge[0], edge[1]));
            max_edge = std::cmp::max(max_edge, std::cmp::max(edge[0], edge[1]));
            adj_list
                .entry(edge[0])
                .and_modify(|e| e.push(edge[1]))
                .or_insert(vec![edge[1]]);
        }
        let mut indegrees: Vec<i32> = vec![0; (max_edge + 1) as usize];
        for edge in edges {
            indegrees[edge[1] as usize] += 1;
        }
        let mut visiting: HashSet<i32> = HashSet::new();
        let mut visited: HashSet<i32> = HashSet::new();
        for (index, indegree) in indegrees.iter().enumerate() {
            if index == 0 {
                continue;
            }
            if *indegree == 0 {
                if is_cycle(index as i32, &adj_list, &mut visiting, &mut visited) {
                    return true;
                }
            }
        }
        false
    }

    pub fn is_undir_graph_acyclic(edges: Vec<Vec<i32>>) -> bool {
        fn is_cycle(
            node: i32,
            parent: Option<i32>,
            adj_list: &AdjList,
            visited: &mut HashSet<i32>,
        ) -> bool {
            visited.insert(node);
            if let Some(neighbours) = adj_list.get(&node) {
                for neighbour in neighbours {
                    if !visited.contains(neighbour) {
                        return is_cycle(*neighbour, Some(node), adj_list, visited);
                    } else {
                        if let Some(parent) = parent {
                            if *neighbour != parent {
                                return true;
                            }
                        }
                    }
                }
            }
            false
        }

        let mut adj_list: AdjList = HashMap::new();
        for edge in edges {
            {
                adj_list
                    .entry(edge[0])
                    .and_modify(|e| e.push(edge[1]))
                    .or_insert(vec![edge[1]]);
                adj_list
                    .entry(edge[1])
                    .and_modify(|e| e.push(edge[0]))
                    .or_insert(vec![edge[0]]);
            }
        }
        let mut visited: HashSet<i32> = HashSet::new();
        for key in adj_list.keys() {
            if !visited.contains(key) {
                if is_cycle(*key, None, &adj_list, &mut visited) {
                    return true;
                }
            }
        }
        false
    }

    pub fn is_cycle_with_union_find(edges: Vec<Vec<i32>>) -> bool {
        let max_value = edges.iter().flatten().max().unwrap();
        let mut union_find = UnionFind::new(*max_value as usize + 1);
        for edge in edges {
            let start = edge[0];
            let end = edge[1];
            let result = union_find.union(start as usize, end as usize);
            if result {
                return true;
            }
        }
        false
    }
}

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
        if x == self.root[x] {
            return x;
        }
        self.root[x] = self.find(self.root[x]);
        self.root[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return true;
        }
        if self.rank[root_x] > self.rank[root_y] {
            self.root[root_y] = root_x;
        } else if self.rank[root_y] > self.rank[root_x] {
            self.root[root_x] = root_y;
        } else {
            self.rank[root_x] += 1;
            self.rank[root_y] = root_x;
        }
        false
    }
}
