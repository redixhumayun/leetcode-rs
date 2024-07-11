use std::collections::HashMap;

use crate::Solution;

#[derive(Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

struct Edge {
    weight: i32,
    point_1: Point,
    point_2: Point,
}

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

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return false; //  already part of the same component
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
    fn set_point_to_index(points: &Vec<Vec<i32>>, point_to_index: &mut HashMap<Point, usize>) {
        for (index, point) in points.iter().enumerate() {
            let point = Point {
                x: point[0],
                y: point[1],
            };
            point_to_index.insert(point, index);
        }
    }

    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let mut edges: Vec<Edge> = Vec::new();
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                let point_1 = Point {
                    x: points[i][0],
                    y: points[i][1],
                };
                let point_2 = Point {
                    x: points[j][0],
                    y: points[j][1],
                };
                let weight = (point_1.x - point_2.x).abs() + (point_1.y - point_2.y).abs();
                edges.push(Edge {
                    weight,
                    point_1,
                    point_2,
                });
            }
        }
        edges.sort_by(|edge_1, edge_2| edge_1.weight.cmp(&edge_2.weight));

        let mut point_to_index: HashMap<Point, usize> = HashMap::new();
        Solution::set_point_to_index(&points, &mut point_to_index);

        let mut union_find = UnionFind::new(points.len());
        let mut num_of_edges = 0;
        let mut cost = 0;
        for edge in edges {
            if num_of_edges == points.len() - 1 {
                break;
            }
            let Edge {
                weight,
                point_1,
                point_2,
            } = edge;
            let point_1_index = point_to_index.get(&point_1).unwrap();
            let point_2_index = point_to_index.get(&point_2).unwrap();
            if union_find.union(*point_1_index, *point_2_index) {
                num_of_edges += 1;
                cost += weight;
            }
        }
        cost
    }
}
