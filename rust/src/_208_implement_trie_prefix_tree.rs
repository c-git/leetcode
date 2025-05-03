//! Solution for https://leetcode.com/problems/implement-trie-prefix-tree
//! 208. Implement Trie (Prefix Tree)

use std::collections::HashMap;

#[derive(Default)]
pub struct Trie {
    root_node: Node,
}

#[derive(Default)]
struct Node {
    /// True if a word ends at this node
    is_terminal: bool,
    next_node: HashMap<char, Node>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, word: String) {
        let mut curr_node = &mut self.root_node;
        for c in word.chars() {
            curr_node = curr_node.next_node.entry(c).or_default();
        }
        curr_node.is_terminal = true;
    }

    pub fn search(&self, word: String) -> bool {
        if let Some(node) = self.last_node_for_chars(word.chars()) {
            node.is_terminal
        } else {
            false
        }
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        self.last_node_for_chars(prefix.chars()).is_some()
    }

    fn last_node_for_chars(&self, chars: impl Iterator<Item = char>) -> Option<&Node> {
        let mut curr_node = &self.root_node;
        for c in chars {
            curr_node = curr_node.next_node.get(&c)?;
        }
        Some(curr_node)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut trie = Trie::new();
        trie.insert("apple".to_owned());
        assert!(trie.search("apple".to_owned())); // return True
        assert!(!trie.search("app".to_owned())); // return False
        assert!(trie.starts_with("app".to_owned())); // return True
        trie.insert("app".to_owned());
        assert!(trie.search("app".to_owned())); // return True
    }
}
