use std::collections::{BTreeMap, VecDeque};

use crate::Solution;

#[derive(Debug)]
pub struct TrieNode {
    char: Option<char>,
    children: BTreeMap<char, TrieNode>,
    is_end: bool,
}

pub struct Trie {
    head: TrieNode,
}

impl Trie {
    pub fn insert(&mut self, word: &str) {
        let mut head = &mut self.head;
        for (index, char) in word.chars().enumerate() {
            let is_last = index == word.len() - 1;
            head = head.children.entry(char).or_insert(TrieNode {
                char: Some(char),
                children: BTreeMap::new(),
                is_end: is_last,
            });
        }
    }

    pub fn search(&self, prefix: &str) -> Vec<String> {
        let mut node = &self.head;
        let mut current = String::new();
        for char in prefix.chars() {
            if let Some(child) = node.children.get(&char) {
                node = child;
                current.push(child.char.unwrap());
            } else {
                return Vec::new();
            }
        }

        fn bfs(node: &TrieNode, prefix: String) -> Vec<String> {
            let mut output = Vec::new();
            let mut queue = VecDeque::new();
            queue.push_back((node, prefix));
            while let Some((current_node, prefix)) = queue.pop_front() {
                if current_node.is_end {
                    output.push(prefix.clone());
                    if output.len() == 3 {
                        break;
                    }
                }

                for (char, child_node) in &current_node.children {
                    let mut new_prefix = prefix.clone();
                    new_prefix.push(*char);
                    queue.push_back((child_node, new_prefix));
                }
            }
            output
        }
        let results = bfs(node, current);
        results

        //  starting from this node, do an in-order traversal
        // let mut results = Vec::new();
        // fn dfs(node: &TrieNode, current: &mut String, results: &mut Vec<String>) {
        //     if node.is_end {
        //         results.push(current.clone());
        //     }
        //     if node.children.len() == 0 || results.len() == 3 {
        //         return;
        //     }

        //     for (char, child_node) in &node.children {
        //         current.push(*char);
        //         dfs(&child_node, current, results);
        //         current.pop();
        //     }
        // }
        // dfs(node, &mut current, &mut results);
        // results
    }
}

impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        //  build a trie with all the product words
        let mut trie = Trie {
            head: TrieNode {
                char: None,
                children: BTreeMap::new(),
                is_end: false,
            },
        };
        for product in products {
            trie.insert(&product);
        }

        //  for every character in the search word, traverse the trie
        let mut search_results: Vec<Vec<String>> = Vec::new();
        let mut prefix = String::new();
        for char in search_word.chars() {
            prefix.push(char);
            search_results.push(trie.search(&prefix));
        }
        search_results
    }
}
