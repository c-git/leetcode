use std::collections::HashMap;

#[derive(Default, Debug)]
struct FoundStatus {
    search_id: usize,
    freq: usize,
    target_freq: usize,
}

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        Self::_find_substring(&s[..], words)
            .into_iter()
            .map(|x| x as i32)
            .collect()
    }

    fn _find_substring(s: &str, words: Vec<String>) -> Vec<usize> {
        let mut result = vec![];
        let mut search_map: HashMap<String, FoundStatus> = HashMap::new();
        let word_len = words[0].len();
        for word in words {
            search_map.entry(word).or_default().target_freq += 1;
        }

        for (i, _) in s.char_indices() {
            if Self::find_substrings_at(i, &s[i..], &mut search_map, word_len) {
                result.push(i);
            }
        }
        result
    }

    fn find_substrings_at(
        search_id: usize,
        mut s: &str,
        search_map: &mut HashMap<String, FoundStatus>,
        word_len: usize,
    ) -> bool {
        let mut complete_count = 0;
        let target_completed = search_map.len();
        while word_len <= s.len() {
            if let Some(found_status) = search_map.get_mut(&s[..word_len]) {
                if found_status.search_id != search_id {
                    found_status.search_id = search_id;
                    found_status.freq = 0;
                }
                found_status.freq += 1;
                match found_status.freq.cmp(&found_status.target_freq) {
                    std::cmp::Ordering::Less => {} // Do nothing maybe there are more
                    std::cmp::Ordering::Equal => {
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
