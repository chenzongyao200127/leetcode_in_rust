// 208_Implement_Trie_(Prefix_Tree)
// https://leetcode.cn/problems/implement-trie-prefix-tree/description/

use std::collections::HashMap;

// Define a Trie node.
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
}

// Implement TrieNode.
impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end_of_word: false,
        }
    }
}

// Define the Trie.
pub struct Trie {
    root: TrieNode,
}

// Implement Trie.
impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    // Insert a word into the Trie.
    pub fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for c in word.chars() {
            node = node.children.entry(c).or_insert_with(TrieNode::new);
        }
        node.is_end_of_word = true;
    }

    // Search for a word in the Trie.
    pub fn search(&self, word: String) -> bool {
        let mut node = &self.root;
        for c in word.chars() {
            match node.children.get(&c) {
                Some(n) => node = n,
                None => return false,
            }
        }
        node.is_end_of_word
    }

    // Check if any word in the Trie starts with the given prefix.
    pub fn starts_with(&self, prefix: String) -> bool {
        let mut node = &self.root;
        for c in prefix.chars() {
            match node.children.get(&c) {
                Some(n) => node = n,
                None => return false,
            }
        }
        true
    }
}
