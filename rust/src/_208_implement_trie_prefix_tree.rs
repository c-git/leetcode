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
                    // current_node = &mut current_node.children[i]; // This way works works
                    current_node = element;
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

    fn search(&self, word: String) -> bool {
        let mut current_node = &self.root;

        'outer: for b in word.bytes() {
            for element in current_node.children.iter() {
                if element.byte == b {
                    current_node = element;
                    continue 'outer;
                }
            }
            return false;
        }

        current_node.is_end_of_word
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
        trie.insert("app".to_owned());
        assert!(trie.search("app".to_owned())); // return True
    }
}

// error[E0499]: cannot borrow `current_node.children` as mutable more than once at a time
//   --> src/_208_implement_trie_prefix_tree.rs:43:13
//    |
// 34 |             for (i, element) in current_node.children.iter_mut().enumerate() {
//    |                                 -------------------------------- first mutable borrow occurs here
// ...
// 43 |             current_node.children.push(TrieNode::new(b));
//    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//    |             |
//    |             second mutable borrow occurs here
//    |             first borrow later used here

// error[E0499]: cannot borrow `current_node.children` as mutable more than once at a time
//   --> src/_208_implement_trie_prefix_tree.rs:44:28
//    |
// 34 |               for (i, element) in current_node.children.iter_mut().enumerate() {
//    |                                   -------------------------------- first mutable borrow occurs here
// ...
// 44 |               current_node = current_node
//    |  ____________________________^
// 45 | |                 .children
// 46 | |                 .last_mut()
//    | |                           ^
//    | |                           |
//    | |___________________________second mutable borrow occurs here
//    |                             first borrow later used here

// For more information about this error, try `rustc --explain E0499`.
// warning: `rust` (lib) generated 1 warning
// error: could not compile `rust` due to 2 previous errors; 1 warning emitted
