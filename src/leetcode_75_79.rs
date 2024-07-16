use std::collections::HashSet;

use crate::Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        fn is_valid(row_index: i32, col_index: i32, board: &Vec<Vec<char>>) -> bool {
            if row_index < 0 || row_index >= board.len() as i32 {
                return false;
            }
            if col_index < 0 || col_index >= board[0].len() as i32 {
                return false;
            }
            return true;
        }

        fn backtrack(
            row_index: i32,
            col_index: i32,
            board: &Vec<Vec<char>>,
            word: &Vec<char>,
            word_index: usize,
            visited: &mut HashSet<(i32, i32)>,
        ) -> bool {
            if word_index == word.len() {
                return true;
            }
            let neighbours: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (-1, 0), (1, 0)];
            let mut result = false;
            for (dx, dy) in neighbours {
                let new_row_index = row_index + dx;
                let new_col_index = col_index + dy;

                if !is_valid(new_row_index, new_col_index, board) {
                    continue;
                }
                if visited.contains(&(new_row_index, new_col_index)) {
                    continue;
                }
                if board[new_row_index as usize][new_col_index as usize] != word[word_index] {
                    continue;
                }
                visited.insert((new_row_index, new_col_index));
                result = result
                    || backtrack(
                        new_row_index,
                        new_col_index,
                        board,
                        word,
                        word_index + 1,
                        visited,
                    );
            }
            result
        }

        //  find all the starting points on the board
        let word: Vec<char> = word.chars().collect();
        let mut starting_points: HashSet<(usize, usize)> = HashSet::new();
        for (row_index, row) in board.iter().enumerate() {
            for (col_index, char) in row.iter().enumerate() {
                if *char == word[0] {
                    starting_points.insert((row_index, col_index));
                }
            }
        }
        let mut visited = HashSet::new();
        for (row_index, col_index) in starting_points {
            visited.clear();
            let result = backtrack(
                row_index as i32,
                col_index as i32,
                &board,
                &word,
                1,
                &mut visited,
            );
            if result == true {
                return true;
            }
        }
        false
    }
}
