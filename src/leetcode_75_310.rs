use std::collections::{HashMap, HashSet, VecDeque};

use crate::Solution;

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        fn top_sort(
            adj_list: &HashMap<i32, Vec<i32>>,
            degrees: &mut HashMap<i32, usize>,
            starting_points: &Vec<i32>,
            n: i32,
        ) -> Vec<i32> {
            let mut queue: VecDeque<i32> = VecDeque::new();
            let mut visited: HashSet<i32> = HashSet::new();
            for &start_point in starting_points {
                queue.push_back(start_point);
                visited.insert(start_point);
                degrees.remove(&start_point);
            }

            let mut remaining_nodes = n as usize;
            while remaining_nodes > 2 {
                remaining_nodes -= queue.len();
                for _ in 0..queue.len() {
                    if let Some(node) = queue.pop_front() {
                        if let Some(neighbours) = adj_list.get(&node) {
                            for &neighbour in neighbours {
                                if visited.contains(&neighbour) {
                                    continue;
                                }
                                visited.insert(neighbour);
                                degrees.entry(neighbour).and_modify(|e| *e -= 1);
                                if *degrees.get(&neighbour).unwrap() == 1 {
                                    queue.push_back(neighbour);
                                }
                            }
                        }
                    }
                }
            }
            queue.into_iter().collect()
        }

        if n == 1 {
            return [0].to_vec();
        }
        let mut adj_list: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut degrees: HashMap<i32, usize> = HashMap::new();
        for edge in edges {
            let first = edge[0];
            let second = edge[1];
            adj_list
                .entry(first)
                .and_modify(|e| e.push(second))
                .or_insert(vec![second]);
            adj_list
                .entry(second)
                .and_modify(|e| e.push(first))
                .or_insert(vec![first]);
            degrees.entry(first).and_modify(|e| *e += 1).or_insert(1);
            degrees.entry(second).and_modify(|e| *e += 1).or_insert(1);
        }
        let start_points: Vec<i32> = degrees
            .iter()
            .filter(|(_, &degree)| degree == 1)
            .map(|(node, _)| *node)
            .collect();
        let result = top_sort(&adj_list, &mut degrees, &start_points, n);
        result
    }
}

/*
building adj list -> O(v+e) with n nodes and (n-1) edges O(n+n-1) ~ O(n)
finding degrees and start points -> O(n) for n nodes
top sort -> O(v+e) because each node is visited once
*/
