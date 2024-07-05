use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.len() == 0 {
            return Vec::new();
        }

        let mut mapping: HashMap<char, Vec<char>> = HashMap::new();
        mapping.insert('2', vec!['a', 'b', 'c']);
        mapping.insert('3', vec!['d', 'e', 'f']);
        mapping.insert('4', vec!['g', 'h', 'i']);
        mapping.insert('5', vec!['j', 'k', 'l']);
        mapping.insert('6', vec!['m', 'n', 'o']);
        mapping.insert('7', vec!['p', 'q', 'r', 's']);
        mapping.insert('8', vec!['t', 'u', 'v']);
        mapping.insert('9', vec!['w', 'x', 'y', 'z']);

        let mut output: Vec<String> = Vec::new();
        fn backtrack(
            input: &Vec<char>,
            curr: &mut Vec<char>,
            i: usize,
            output: &mut Vec<String>,
            mapping: &HashMap<char, Vec<char>>,
        ) {
            if curr.len() == input.len() {
                output.push(curr.iter().collect());
                return;
            }
            if i >= input.len() {
                return;
            }
            let char = input[i];
            for letter in mapping.get(&char).unwrap() {
                curr.push(*letter);
                backtrack(input, curr, i + 1, output, mapping);
                curr.pop();
            }
        }
        backtrack(
            &digits.chars().collect(),
            &mut Vec::new(),
            0,
            &mut output,
            &mapping,
        );
        output
    }
}
