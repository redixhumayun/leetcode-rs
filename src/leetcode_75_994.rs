use std::collections::{HashSet, VecDeque};

use crate::Solution;

type Node = (usize, usize);
type QueueValue = (Node, i32);
impl Solution {
    pub fn is_valid(grid: &Vec<Vec<i32>>, node: Node) -> bool {
        let (row, col) = node;
        if row < 0 || row >= grid.len() {
            return false;
        }
        if col < 0 || col >= grid[0].len() {
            return false;
        }
        if grid[row][col] == 0 {
            return false;
        }
        true
    }

    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut queue: VecDeque<QueueValue> = VecDeque::new();
        let mut visited: HashSet<Node> = HashSet::new();
        for (index, row) in grid.iter().enumerate() {
            for (col_index, col) in grid.get(index).unwrap().iter().enumerate() {
                if grid[index][col_index] == 2 {
                    queue.push_back(((index, col_index), 0));
                }
            }
        }

        let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut max_steps = 0;
        let mut grid = grid;
        while let Some((node, steps)) = queue.pop_front() {
            max_steps = std::cmp::max(max_steps, steps);
            visited.insert(node);
            let (x, y) = node;
            for (dx, dy) in directions {
                let new_x = (x as i32 + dx) as usize;
                let new_y = (y as i32 + dy) as usize;
                if !Solution::is_valid(&grid, (new_x, new_y)) {
                    continue;
                }
                if visited.contains(&(new_x, new_y)) {
                    continue;
                }
                queue.push_back(((new_x, new_y), steps + 1));
                grid[new_x][new_y] = 2;
            }
        }

        for (row_index, _) in grid.iter().enumerate() {
            for (col_index, _) in grid.get(row_index).iter().enumerate() {
                if grid[row_index][col_index] == 1 {
                    return -1;
                }
            }
        }
        max_steps
    }
}
