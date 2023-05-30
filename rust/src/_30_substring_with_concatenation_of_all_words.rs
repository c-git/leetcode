use std::collections::HashMap;

#[derive(Default, Debug)]
struct FoundStatus {
    /// This ID allows us to check without clearing if this item has been found during this search
    search_id: usize,
    /// This represents the frequency of the item found during the search associated with the ID
    freq: usize,
    /// This is the number of times this item showed up in the original list
    target_freq: usize,
}

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        // Call helper function with better signature and then map results back to this signature
        Self::_find_substring(&s[..], words)
            .into_iter()
            .map(|x| x as i32)
            .collect()
    }

    /// Helper method with better signature that actually does the work
    fn _find_substring(s: &str, words: Vec<String>) -> Vec<usize> {
        let mut result = vec![];
        let word_len = words[0].len(); // Store length of the words to prevent trying to get it repeatedly

        // Store the words and their frequencies in a hashmap for quick lookup
        let mut search_map: HashMap<String, FoundStatus> = HashMap::new();
        for word in words {
            search_map.entry(word).or_default().target_freq += 1;
        }

        // Step through each position of the string and check if all words can be found starting from there
        for (i, _) in s.char_indices() {
            if Self::find_substrings_at(i, &s[i..], &mut search_map, word_len) {
                result.push(i);
            }
        }
        result
    }

    /// Checks if all the words can be found at the beginning of the string passed
    fn find_substrings_at(
        search_id: usize, // Uniquely identifies this search (values defaulted to 0 but with 0 freq so it's fine)
        mut s: &str,      //The string to search in
        search_map: &mut HashMap<String, FoundStatus>, // The map to use to do the searching
        word_len: usize,
    ) -> bool {
        let mut complete_count = 0; // How many words have we found so far (full multiplicity of a word must be found be for it can be marked completed)
        let target_completed = search_map.len(); // How many need to be completed to say that we have a match
        while word_len <= s.len() {
            if let Some(found_status) = search_map.get_mut(&s[..word_len]) {
                if found_status.search_id != search_id {
                    // Clear old results
                    found_status.search_id = search_id;
                    found_status.freq = 0;
                }
                found_status.freq += 1;
                match found_status.freq.cmp(&found_status.target_freq) {
                    std::cmp::Ordering::Less => {} // Do nothing maybe there are more
                    std::cmp::Ordering::Equal => {
                        // Full frequency of word found
                        complete_count += 1;
                        if complete_count == target_completed {
                            return true;
                        }
                    }
                    std::cmp::Ordering::Greater => {
                        return false; // Too many found can no longer match
                    }
                }
            } else {
                return false; // No match found, cannot match anymore
            }
            s = &s[word_len..]; // Move onto next word
        }
        false
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("barfoothefoobarman", vec!["foo","bar"],vec![0,9])]
    #[case("wordgoodgoodgoodbestword", vec!["word","good","best","word"],vec![])]
    #[case("barfoofoobarthefoobarman", vec!["bar","foo","the"], vec![6,9,12])]
    #[case("wordgoodgoodwordgoodbestwordword", vec!["word","good","best","word"],vec![12,16])]
    fn case(#[case] s: String, #[case] words: Vec<&str>, #[case] expected: Vec<i32>) {
        let words = words.into_iter().map(|x| x.to_string()).collect();
        let actual = Solution::find_substring(s, words);
        assert_eq!(actual, expected);
    }
}
