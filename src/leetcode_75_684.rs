use std::collections::{HashMap, HashSet};

use crate::Solution;

type AdjList = HashMap<i32, Vec<i32>>;
type VisitedSet = HashSet<i32>;

struct UnionFind {
    root: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        let root = (0..(size + 1) as usize).into_iter().collect();
        let rank = vec![1; (size + 1) as usize];
        Self { root, rank }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.root[x] == x {
            return x;
        }
        self.root[x] = self.find(self.root[x]);
        return self.root[x];
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }
        if self.rank[root_x] > self.rank[root_y] {
            self.root[root_y] = root_x;
        } else if self.rank[root_y] > self.rank[root_x] {
            self.root[root_x] = root_y;
        } else {
            self.root[root_y] = root_x;
            self.rank[root_x] += 1;
        }
        return true;
    }
}

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut union_find = UnionFind::new(edges.len());
        for edge in edges {
            if !union_find.union(edge[0] as usize, edge[1] as usize) {
                return vec![edge[0], edge[1]];
            }
        }
        Vec::new()
        // fn dfs(node: i32, parent: i32, visited: &mut VisitedSet, adj_list: &AdjList) -> Vec<i32> {
        //     if visited.contains(&node) {
        //         panic!("self loop detected");
        //     }
        //     if let Some(neighbours) = adj_list.get(&node) {
        //         for neighbour in neighbours {
        //             if !visited.contains(neighbour) {
        //                 visited.insert(*neighbour);
        //                 dfs(*neighbour, node, visited, adj_list);
        //             } else if *neighbour != parent {
        //                 //  cycle detected
        //                 return vec![*neighbour, node];
        //             }
        //         }
        //     }
        //     panic!("no cycle found");
        // }

        // //  build adjacency list
        // let mut adj_list: AdjList = HashMap::new();
        // for edge in edges {
        //     adj_list
        //         .entry(edge[0])
        //         .and_modify(|e| e.push(edge[1]))
        //         .or_insert(vec![edge[1]]);
        //     adj_list
        //         .entry(edge[1])
        //         .and_modify(|e| e.push(edge[0]))
        //         .or_insert(vec![edge[0]]);
        // }
        // let mut visited: VisitedSet = HashSet::new();
        // let result = dfs(1, 0, &mut visited, &adj_list);
        // result
    }
}
