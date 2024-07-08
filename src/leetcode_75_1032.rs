use std::collections::HashMap;

#[derive(Debug)]
pub struct TrieNode {
    char: Option<char>,
    children: HashMap<char, TrieNode>,
    is_end: bool,
}

pub struct Trie {
    head: TrieNode,
}

pub struct StreamChecker {
    trie: Trie,
    prefix: String,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StreamChecker {
    pub fn new(words: Vec<String>) -> Self {
        //  iterate over all the words in reverse and build a trie out of it
        let mut head = TrieNode {
            char: None,
            children: HashMap::new(),
            is_end: false,
        };
        for word in words {
            let mut current = &mut head;
            for (index, char) in word.chars().into_iter().rev().enumerate() {
                let is_end = index == word.len() - 1;
                current = current.children.entry(char).or_insert(TrieNode {
                    char: Some(char),
                    children: HashMap::new(),
                    is_end,
                });
            }
        }
        StreamChecker {
            trie: Trie { head },
            prefix: String::new(),
        }
    }

    fn search(&self, prefix: &str) -> bool {
        let mut head = &self.trie.head;
        for (_, char) in prefix.chars().enumerate() {
            if let Some(node) = head.children.get(&char) {
                head = node;
                if node.is_end {
                    return true;
                }
            } else {
                return false;
            }
        }
        return false;
    }

    pub fn query(&mut self, letter: char) -> bool {
        self.prefix.insert(0, letter);
        self.search(&self.prefix)
    }
}
