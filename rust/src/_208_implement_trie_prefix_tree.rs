//! Source: <https://dev.to/timclicks/two-trie-implementations-in-rust-ones-super-fast-2f3m>
//! Wanted to test out some of the ideas he proposed in his video https://www.youtube.com/live/f9B87LA86g0

#[derive(Default)]
struct TrieNode {
    is_end_of_word: bool,
    children: Vec<(
        u8, /* byte to match (for this problem based on constraints will be = char except 1/4 the space) */
        TrieNode,
    )>,
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

        'outer: for b in word.bytes() {
            for (i, element) in current_node.children.iter_mut().enumerate() {
                if element.0 == b {
                    current_node = &mut current_node.children[i].1;
                    continue 'outer;
                }
            }
            let n = current_node.children.len(); // Will be n - 1 after push
            current_node.children.push((b, Default::default()));
            current_node = &mut current_node.children[n].1;
        }
        current_node.is_end_of_word = true;
    }
    fn find_prefix_node(&self, word: &str) -> Option<&TrieNode> {
        let mut current_node = &self.root;

        'outer: for b in word.bytes() {
            for element in current_node.children.iter() {
                if element.0 == b {
                    current_node = &element.1;
                    continue 'outer;
                }
            }
            return None;
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
