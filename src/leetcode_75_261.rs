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

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return false; //  these nodes are already connected, so a cycle
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
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        if edges.len() != (n - 1) as usize {
            return false;
        }
        let mut union_find = UnionFind::new(n as usize);
        for edge in edges {
            let start = edge[0];
            let end = edge[1];
            let result = union_find.union(start as usize, end as usize);
            if !result {
                return false;
            }
        }
        true
    }
}
