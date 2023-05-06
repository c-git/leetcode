use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::Debug;
use std::iter::once;
use std::rc::Rc;

type Word = Rc<String>;

#[derive(Hash, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct CountID(usize);

impl AsRef<usize> for CountID {
    fn as_ref(&self) -> &usize {
        &self.0
    }
}

impl Debug for CountID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Only provides information of if the next/prev key value exists
#[derive(Default, Debug)]
struct HashList {
    data: BTreeMap<CountID, HashSet<Word>>,
}

impl HashList {
    fn count_num_words_as_str(&self) -> String {
        if self.data.is_empty() {
            "".to_string()
        } else {
            let mut result = String::with_capacity(self.data.len() * 11);
            for (count_id, set) in self.data.iter() {
                result.push_str(&format!("{} ({}) -> ", count_id.as_ref(), set.len()));
            }
            result
        }
    }

    fn decrement_word(&mut self, word: &Word, current_id: CountID) -> Option<CountID> {
        let new_count = CountID(current_id.as_ref() - 1);
        self.move_word_to_new_id(new_count, current_id, word);
        match new_count {
            CountID(0) => None,
            val => Some(val),
        }
    }

    fn increment_word(&mut self, word: &Word, current_id: CountID) -> CountID {
        let new_count = CountID(current_id.as_ref() + 1);
        self.move_word_to_new_id(new_count, current_id, word);
        new_count
    }

    fn move_word_to_new_id(&mut self, new_id: CountID, old_id: CountID, word: &Word) {
        //! Creates or updates the set for `new_count` unless it is 0
        match self.data.get_mut(&new_id) {
            Some(set) => {
                let newly_inserted = set.insert(Rc::clone(word));
                debug_assert!(newly_inserted, "Should not be a new value to the new set");
            }
            None => {
                let newly_inserted = self.data.insert(new_id, once(Rc::clone(word)).collect());
                debug_assert!(
                    newly_inserted.is_none(),
                    "Should not be a new value to the new set"
                );
            }
        }

        let old_set = self.data.get_mut(&old_id).expect("Must exist in map");
        old_set.remove(word);
        if old_set.is_empty() {
            self.data.remove(&old_id);
        }
    }

    fn insert_new(&mut self, word: Word) -> CountID {
        let new_id = CountID(1);
        match self.data.get_mut(&new_id) {
            Some(set) => {
                let newly_inserted = set.insert(Rc::clone(&word));
                debug_assert!(newly_inserted, "Should not be a new value to the new set");
            }
            None => {
                let newly_inserted = self.data.insert(new_id, once(Rc::clone(&word)).collect());
                debug_assert!(
                    newly_inserted.is_none(),
                    "Should not be a new value to the new set"
                );
            }
        }
        new_id
    }

    fn get_max_key(&self) -> Option<String> {
        self.data.iter().rev().next().map(|(_, set)| {
            set.iter()
                .next()
                .expect("Empty Sets Not allowed")
                .to_string()
        })
    }

    fn get_min_key(&self) -> Option<String> {
        self.data.iter().next().map(|(_, set)| {
            set.iter()
                .next()
                .expect("Empty Sets Not allowed")
                .to_string()
        })
    }
}

#[derive(Default)]
struct AllOne {
    /// Stores the list of id's that have a count and the words with that count
    count_list: HashList,

    /// Stores a mapping from a word to the id in the list that has that word in it
    items_map: HashMap<Word, CountID>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AllOne {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn inc(&mut self, key: String) {
        let word = Rc::new(key);
        let new_id = match self.items_map.get(&word) {
            Some(id) => self.count_list.increment_word(&word, *id),
            None => self.count_list.insert_new(Rc::clone(&word)),
        };
        self.items_map.insert(word, new_id);
    }

    pub fn dec(&mut self, key: String) {
        let word = Rc::new(key);
        let old_id = self
            .items_map
            .get(&word)
            .expect("Based on constraints in the question");

        if let Some(new_id) = self.count_list.decrement_word(&word, *old_id) {
            self.items_map.insert(word, new_id);
        } else {
            self.items_map.remove(&word);
        }
    }

    pub fn get_max_key(&self) -> String {
        self.count_list.get_max_key().unwrap_or_default()
    }

    pub fn get_min_key(&self) -> String {
        self.count_list.get_min_key().unwrap_or_default()
    }
}

/**
 * Your AllOne object will be instantiated and called as such:
 * let obj = AllOne::new();
 * obj.inc(key);
 * obj.dec(key);
 * let ret_3: String = obj.get_max_key();
 * let ret_4: String = obj.get_min_key();
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut all_one = AllOne::new();
        all_one.inc("hello".into());
        all_one.inc("hello".into());
        assert_eq!(all_one.get_max_key(), "hello"); // return "hello"
        assert_eq!(all_one.get_min_key(), "hello"); // return "hello"
        all_one.inc("leet".into());
        assert_eq!(all_one.get_max_key(), "hello"); // return "hello"
        assert_eq!(all_one.get_min_key(), "leet"); // return "leet"
    }

    #[test]
    fn case2() {
        let mut all_one = AllOne::new();
        all_one.inc("hello".into());
        all_one.inc("goodbye".into());
        all_one.inc("hello".into());
        all_one.inc("hello".into());
        all_one.get_max_key();
        all_one.inc("leet".into());
        all_one.inc("code".into());
        all_one.dec("leet".into());
        all_one.inc("hello".into());
        all_one.inc("leet".into());
        all_one.inc("code".into());
        all_one.inc("code".into());
        all_one.get_max_key();
    }

    #[test]
    fn case3() {
        let mut obj = AllOne::new();
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("hello".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("goodbye".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("hello".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("hello".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        assert_eq!(obj.get_max_key(), "hello");
        dbg!(obj.inc("leet".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("code".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("leet".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.dec("hello".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("leet".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("code".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("code".into()));
        println!(
            // Print map values
            "{:?}",
            obj.items_map
        );
        println!("{:?}", obj.count_list.count_num_words_as_str());
        assert!(["leet", "code"].contains(&&obj.get_max_key()[..]));
    }

    #[test]
    fn case4() {
        let mut obj = AllOne::new();
        dbg!(obj.inc("a".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("b".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("c".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("d".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("a".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("b".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("c".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("d".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("c".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("d".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("d".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("a".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        assert_eq!(obj.get_min_key(), "b");
    }

    #[test]
    fn case5() {
        let mut obj = AllOne::new();
        obj.inc("hello".into());
        obj.inc("l".into());
        obj.inc("l".into());
        obj.inc("l".into());
        obj.inc("k".into());
        obj.inc("k".into());
        obj.inc("k".into());
        obj.inc("j".into());
        obj.inc("j".into());
        obj.inc("j".into());
        obj.dec("j".into());
        obj.dec("k".into());
        obj.get_max_key();
    }
}
