//! Source: <https://dev.to/timclicks/two-trie-implementations-in-rust-ones-super-fast-2f3m>
//! Wanted to test out some of the ideas he proposed in his video https://www.youtube.com/live/f9B87LA86g0

use std::collections::HashMap;

#[derive(Default)]
struct TrieNode {
    is_end_of_word: bool,
    children: HashMap<char, TrieNode>,
}

#[derive(Default)]
struct Trie {
    root: TrieNode,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, word: String) {
        let mut current_node = &mut self.root;

        for c in word.chars() {
            current_node = current_node.children.entry(c).or_default();
        }
        current_node.is_end_of_word = true;
    }
    fn find_prefix_node(&self, word: &str) -> Option<&TrieNode> {
        let mut current_node = &self.root;

        for c in word.chars() {
            match current_node.children.get(&c) {
                Some(node) => current_node = node,
                None => return None,
            }
        }

        Some(current_node)
    }

    fn search(&self, word: String) -> bool {
        match self.find_prefix_node(&word) {
            Some(node) => node.is_end_of_word,
            None => false,
        }
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.find_prefix_node(&prefix).is_some()
    }
}

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
