use std::collections::{HashSet, VecDeque};

use crate::Solution;

type Node = (i32, i32);
type QueueValue = (Node, i32);

impl Solution {
    pub fn is_exit(maze: &Vec<Vec<char>>, node: Node, entrance: Node) -> bool {
        let (row, col) = node;
        if node != entrance
            && (row == 0
                || row as usize == maze.len() - 1
                || col == 0
                || col as usize == maze[0].len() - 1)
        {
            return true;
        }
        false
    }

    pub fn is_valid(maze: &Vec<Vec<char>>, node: Node) -> bool {
        let (row, col) = node;
        if row < 0
            || row as usize >= maze.len()
            || col < 0
            || col as usize >= maze[0].len()
            || maze[row as usize][col as usize] == '+'
        {
            return false;
        }
        true
    }

    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let entrance_node = (entrance[0], entrance[1]);
        let mut queue: VecDeque<QueueValue> = VecDeque::new();
        queue.push_back((entrance_node, 0));
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut visited = HashSet::new();
        visited.insert(entrance_node);

        while let Some((node, dist)) = queue.pop_front() {
            if Solution::is_exit(&maze, node, entrance_node) {
                return dist;
            }
            let (x, y) = node;
            for (dx, dy) in directions.iter() {
                let new_x = x + dx;
                let new_y = y + dy;
                let new_node = (new_x, new_y);
                if !visited.contains(&new_node) && Solution::is_valid(&maze, new_node) {
                    queue.push_back((new_node, dist + 1));
                    visited.insert(new_node);
                }
            }
        }
        -1
    }
}
