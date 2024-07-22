use std::collections::{HashMap, HashSet};

use crate::Solution;

impl Solution {
    fn build_rc_to_box_mapping() -> HashMap<(usize, usize), usize> {
        let mut mapping: HashMap<(usize, usize), usize> = HashMap::new();
        mapping.insert((0, 0), 0);
        mapping.insert((0, 1), 1);
        mapping.insert((0, 2), 2);
        mapping.insert((1, 0), 3);
        mapping.insert((1, 1), 4);
        mapping.insert((1, 2), 5);
        mapping.insert((2, 0), 6);
        mapping.insert((2, 1), 7);
        mapping.insert((2, 2), 8);
        mapping
    }

    fn map_rc_to_box(
        r_index: usize,
        c_index: usize,
        mapping: &HashMap<(usize, usize), usize>,
    ) -> usize {
        mapping.get(&(r_index / 3, c_index / 3)).unwrap().clone()
    }

    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let num_of_boxes = board.len() / 3 * board[0].len() / 3;
        let rc_box_mapping = Solution::build_rc_to_box_mapping();
        let mut boxes: HashMap<usize, i32> = HashMap::new();
        let mut cols: HashMap<usize, i32> = HashMap::new();
        for i in 0..board[0].len() {
            cols.insert(i, 0);
        }
        for i in 0..num_of_boxes {
            boxes.insert(i, 0);
        }

        for (row_index, row) in board.iter().enumerate() {
            let mut row_set = 0;
            for (col_index, char) in row.iter().enumerate() {
                if *char == '.' {
                    continue; //  don't bother with empty grid
                }
                let char_digit = *char as u32 - '0' as u32;
                if row_set & (1 << char_digit) != 0 {
                    return false;
                }
                row_set = row_set | (1 << char_digit);
                let col_set = match cols.get_mut(&col_index) {
                    Some(set) => set,
                    None => panic!("error"),
                };
                if *col_set & (1 << char_digit) != 0 {
                    return false;
                }
                *col_set = *col_set | (1 << char_digit);
                let box_index = Solution::map_rc_to_box(row_index, col_index, &rc_box_mapping);
                let box_set = match boxes.get_mut(&box_index) {
                    Some(b) => b,
                    None => panic!("error"),
                };
                if *box_set & (1 << char_digit) != 0 {
                    return false;
                }
                *box_set = *box_set | (1 << char_digit);
            }
        }
        true
    }
}
