// // extern crate rand;
// pub fn generate_3x3_sudoku() -> (Vec<Vec<i32>>, Vec<Vec<Option<i32>>>) {
//     let mut solved = vec![vec![0; 3]; 3];
//     let mut rng = thread_rng();

//     // Helper function to check if placing num at (row, col) is valid
//     fn is_valid(solved: &Vec<Vec<i32>>, row: usize, col: usize, num: i32) -> bool {
//         for i in 0..3 {
//             if solved[row][i] == num || solved[i][col] == num {
//                 return false;
//             }
//         }
//         true
//     }

//     // Backtracking function to fill the 3x3 grid
//     fn fill_grid(solved: &mut Vec<Vec<i32>>, row: usize, col: usize) -> bool {
//         if row == 3 {
//             return true;
//         }
//         if col == 3 {
//             return fill_grid(solved, row + 1, 0);
//         }
//         let mut nums = vec![1, 2, 3];
//         nums.shuffle(&mut thread_rng());

//         for &num in &nums {
//             if is_valid(solved, row, col, num) {
//                 solved[row][col] = num;
//                 if fill_grid(solved, row, col + 1) {
//                     return true;
//                 }
//                 solved[row][col] = 0;
//             }
//         }
//         false
//     }

//     fill_grid(&mut solved, 0, 0);

//     // Copy the solved grid and create the puzzle with 3 hints
//     let mut puzzle = vec![vec![None; 3]; 3];
//     let mut positions: Vec<(usize, usize)> =
//         (0..3).flat_map(|i| (0..3).map(move |j| (i, j))).collect();
//     positions.shuffle(&mut rng);
//     for &(row, col) in &positions[..3] {
//         puzzle[row][col] = Some(solved[row][col]);
//     }

//     (solved, puzzle)
// }

// fn main() {
//     let (solved, puzzle) = generate_3x3_sudoku();
//     println!("Solved 3x3 Sudoku:");
//     for row in &solved {
//         println!("{:?}", row);
//     }
//     println!("\nPuzzle with 3 hints:");
//     for row in &puzzle {
//         println!("{:?}", row);
//     }
// }

use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn generate_3x3_sudoku(size: usize) {
    fn is_valid(solved: &Vec<Vec<i32>>, row: usize, col: usize, num: i32) -> bool {
        let size = solved.len();
        for i in 0..size {
            if solved[row][i] == num || solved[i][col] == num {
                return false;
            }
        }
        return true;
    }

    fn fill_grid(size: usize, row: usize, col: usize, solved: &mut Vec<Vec<i32>>) -> bool {
        if row == size {
            return true; //  base case because entire grid has been generated
        }
        if col == size {
            return fill_grid(size, row + 1, 0, solved);
        }
        let mut vector: Vec<i32> = (1..=size as i32).collect();
        vector.shuffle(&mut thread_rng());
        for num in vector {
            if is_valid(solved, row, col, num) {
                solved[row][col] = num;
                if fill_grid(size, row, col + 1, solved) {
                    return true;
                }
            }
        }
        return false;
    }

    let mut solved = vec![vec![0; size]; size];
    fill_grid(size, 0, 0, &mut solved);
    println!("The grid {:?}", solved);
    let mut output = vec![vec![0; size]; size];
    let mut random_picker = Vec::new();
    for row in 0..size {
        for col in 0..size {
            random_picker.push((row, col));
        }
    }
    random_picker.shuffle(&mut thread_rng());
    for i in 0..3 {
        let (row, col) = random_picker[i];
        output[row][col] = solved[row][col];
    }
    println!("The output {:?}", output);
}
