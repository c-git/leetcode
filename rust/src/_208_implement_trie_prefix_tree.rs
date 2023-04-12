struct TrieNode {
    /// The value that this node was created for (in utf-8 this matches a char)
    /// For the leetcode problem this is the same as the ascii value of the letter
    byte: u8,
    is_end_of_word: bool,
    children: Vec<TrieNode>,
}

impl TrieNode {
    fn new(byte: u8) -> Self {
        Self {
            byte,
            is_end_of_word: false,
            children: vec![],
        }
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Self {
            root: TrieNode::new(0), // The byte at the root does not get used by design just easier to have it than put an option when all the rest have
        }
    }

    fn insert(&mut self, word: String) {
        let mut current_node = &mut self.root;

        'outer: for b in word.bytes() {
            for (i, element) in current_node.children.iter_mut().enumerate() {
                if element.byte == b {
                    current_node = &mut current_node.children[i];
                    continue 'outer;
                }
            }

            // `b` did not already have a node. Add one for `b`
            current_node.children.push(TrieNode::new(b));
            current_node = current_node
                .children
                .last_mut()
                .expect("Item was just added");
        }
        current_node.is_end_of_word = true;
    }

    fn find_prefix_node(&self, word: &str) -> Option<&TrieNode> {
        let mut current_node = &self.root;

        for b in word.bytes() {
            if let Some(node) = current_node.children.iter().find(|node| node.byte == b) {
                current_node = node;
            } else {
                return None;
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
