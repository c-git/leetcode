//! Solution based on Trie in 208

struct TrieNode {
    /// The value that this node was created for (in utf-8 this matches a char)
    /// For the leetcode problem this is the same as the ascii value of the letter
    byte: u8,
    is_end_of_word: bool,
    children: Vec<TrieNode>,
}

impl TrieNode {
    const DOT: u8 = 46; // Matches any value

    fn new(byte: u8) -> Self {
        Self {
            byte,
            is_end_of_word: false,
            children: vec![],
        }
    }

    fn search(&self, word_as_bytes: &[u8]) -> bool {
        if word_as_bytes.is_empty() {
            //If this node is the end then it's a match there are no more bytes to check
            return self.is_end_of_word;
        }

        let mut current_node = self;

        for (i, b) in word_as_bytes.iter().enumerate() {
            if *b == Self::DOT {
                // Match any character
                return current_node
                    .children
                    .iter()
                    .any(|child| child.search(&word_as_bytes[i + 1..]));
            } else if let Some(node) = current_node.children.iter().find(|node| node.byte == *b) {
                current_node = node;
            } else {
                return false;
            }
        }

        current_node.is_end_of_word
    }
}

struct WordDictionary {
    root: TrieNode,
}

impl WordDictionary {
    fn new() -> Self {
        Self {
            root: TrieNode::new(0), // The byte at the root does not get used by design just easier to have it than put an option when all the rest have
        }
    }

    fn add_word(&mut self, word: String) {
        let mut current_node = &mut self.root;

        for b in word.bytes() {
            if let Some((i, _)) = current_node
                .children
                .iter()
                .enumerate()
                .find(|(_, element)| element.byte == b)
            {
                current_node = &mut current_node.children[i];
            } else {
                // `b` did not already have a node. Add one for `b`
                current_node.children.push(TrieNode::new(b));
                current_node = current_node
                    .children
                    .last_mut()
                    .expect("Item was just added");
            }
        }
        current_node.is_end_of_word = true;
    }

    fn search(&self, word: String) -> bool {
        let word_as_bytes = word.as_bytes();
        self.root.search(word_as_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut word_dictionary = WordDictionary::new();
        word_dictionary.add_word("bad".to_string());
        word_dictionary.add_word("dad".to_string());
        word_dictionary.add_word("mad".to_string());
        assert!(!word_dictionary.search("pad".to_string())); // return False
        assert!(word_dictionary.search("bad".to_string())); // return True
        assert!(word_dictionary.search(".ad".to_string())); // return True
        assert!(word_dictionary.search("b..".to_string())); // return True
    }
}
