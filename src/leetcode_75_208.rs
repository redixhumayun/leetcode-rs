use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TrieNode {
    char: Option<char>,
    children: HashMap<char, TrieNode>,
    is_end: bool,
}

impl PartialEq for TrieNode {
    fn eq(&self, other: &Self) -> bool {
        self.char == other.char
    }
}

pub struct Trie {
    head: TrieNode,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    pub fn new() -> Self {
        Trie {
            head: TrieNode {
                char: None,
                children: HashMap::new(),
                is_end: false,
            },
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut head = &mut self.head;
        let chars: Vec<char> = word.chars().collect();
        let mut index = 0;
        while index < chars.len() {
            let char = chars[index];
            let is_last = index == chars.len() - 1;
            head = head.children.entry(char).or_insert(TrieNode {
                char: Some(char),
                children: HashMap::new(),
                is_end: false,
            });
            if is_last {
                head.is_end = true;
            }
            index += 1;
        }
    }

    pub fn search(&self, word: String) -> bool {
        let mut head = &self.head;
        let chars: Vec<char> = word.chars().collect();
        for (index, char) in chars.iter().enumerate() {
            if let Some(node) = head.children.get(char) {
                head = node;
            } else {
                return false;
            }
            if index == chars.len() - 1 {
                if !head.is_end {
                    return false;
                }
            }
        }
        true
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let mut head = &self.head;
        let chars: Vec<char> = prefix.chars().collect();
        for (_, char) in chars.iter().enumerate() {
            if let Some(node) = head.children.get(char) {
                head = node;
            } else {
                return false;
            }
        }
        true
    }
}
