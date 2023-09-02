//! Solution for https://leetcode.com/problems/extra-characters-in-a-string
//! 2707. Extra Characters in a String

use std::collections::{HashMap, HashSet};

struct Trie {
    id: usize,
    children: HashMap<char, Trie>,
    is_end_of_word: bool,
}

impl Trie {
    fn new(id: usize) -> Self {
        Self {
            id,
            children: Default::default(),
            is_end_of_word: Default::default(),
        }
    }
    fn add_word(&mut self, word: &str, next_id: &mut usize) {
        let mut curr = self;
        for c in word.chars() {
            curr = curr.children.entry(c).or_insert(Trie::new(*next_id));
            *next_id += 1;
        }
        curr.is_end_of_word = true;
    }
}

impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let mut result = i32::MAX;

        let mut next_id = 0usize;
        let mut full_trie = Trie::new(next_id);
        next_id += 1;
        dictionary
            .iter()
            .for_each(|x| full_trie.add_word(x, &mut next_id));

        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut seen = HashSet::new();
        let mut stack = vec![(0, 0, true, &full_trie)];
        while let Some((idx, unused, is_at_top_of_trie, trie)) = stack.pop() {
            // LI:
            //    - idx is a valid index into s
            //    - unused holds how many letters have been skipped so far
            //    - letters_matched is how many letters have been matched so far in this current word
            //    - trie is the part of the try remaining after matching previous letters in s[..i]
            let key = (idx, unused, is_at_top_of_trie, trie.id);
            if seen.contains(&key) {
                continue;
            } else {
                seen.insert(key);
            }

            let is_last_letter = idx + 1 >= n;

            if is_at_top_of_trie {
                // Consider the option of skipping this letter
                if is_last_letter {
                    // Try just skipping as unused
                    result = result.min(unused + 1);
                } else {
                    // Try skipping and continuing
                    stack.push((idx + 1, unused + 1, true, &full_trie));
                }
            }

            if let Some(child) = trie.children.get(&s[idx]) {
                match (child.is_end_of_word, is_last_letter) {
                    (true, true) => {
                        // End of the input letters this is one way to use all the letters
                        if unused == 0 {
                            return 0;
                        } else {
                            result = result.min(unused);
                        }
                    }
                    (true, false) => {
                        // This is a word but there are more letters keep
                        // Both keep on trying to match
                        // and consider starting from the beginning again
                        stack.push((idx + 1, unused, true, &full_trie));
                        stack.push((idx + 1, unused, false, child));
                    }
                    (false, true) => (), // letters finished but not the end of a word, unusable path
                    (false, false) => {
                        // Not the end of a word, only option is to move forward
                        stack.push((idx + 1, unused, false, child));
                    }
                }
            }
        }
        result
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("leetscode", vec!["leet".into(),"code".into(),"leetcode".into()], 1)]
    #[case("sayhelloworld", vec!["hello".into(),"world".into()], 3)]
    #[case("nwlztjn", vec!["jm".into(),"z".into(),"l".into(),"w".into()], 4)]
    // #[case("nwlztjn", vec!["a".into(),"f".into(),"v".into(),"me".into(),"m".into(),"bv".into(),"g".into(),"ss".into(),"tu".into(),"jm".into(),"z".into(),"kg".into(),"l".into(),"go".into(),"cn".into(),"uj".into(),"kx".into(),"w".into(),"qz".into(),"e".into(),"ut".into(),"tf".into(),"zn".into(),"ha".into(),"ke".into(),"af".into(),"aj".into(),"ls".into(),"r".into(),"no".into(),"pm".into(),"qn".into(),"yw".into(),"cs".into(),"oz".into(),"b".into()], 4)]
    #[case("dwmodizxvvbosxxw", vec!["ox".into(),"lb".into(),"diz".into(),"gu".into(),"v".into(),"ksv".into(),"o".into(),"nuq".into(),"r".into(),"txhe".into(),"e".into(),"wmo".into(),"cehy".into(),"tskz".into(),"ds".into(),"kzbu".into()], 7)]
    #[case("leetsleet", vec!["leet".into()], 1)]
    #[case("leetleeleet", vec!["leet".into(), "lee".into()], 0)]
    #[case("leetleet", vec!["leet".into()], 0)]
    #[case("le", vec!["lee".into()], 2)]
    #[case("leetleeleete", vec!["lee".into(),"leet".into(),"leeleete".into()], 0)]
    #[case("e", vec!["e".into()], 0)]
    #[case("e", vec!["a".into()], 1)]
    #[case("eeeeee", vec!["e".into()], 0)]
    #[case("eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee", vec!["e".into()], 0)]
    #[case("tyepjjzouutdpglkwicituzpwgwtwrvuyodexpzivrfot", vec!["mbddl".into(),"zpwg".into(),"obkds".into(),"vmbyln".into(),"o".into(),"w".into(),"lkwic".into(),"rkbfvk".into(),"qdfvai".into(),"z".into(),"wtw".into(),"ou".into(),"tdpg".into(),"wa".into(),"lepgn".into(),"it".into(),"chks".into(),"zqev".into(),"bsoer".into(),"m".into(),"cvwzf".into(),"dexp".into(),"jz".into(),"szrt".into(),"yarce".into(),"vsnig".into(),"u".into(),"ot".into(),"rvu".into(),"tzps".into(),"mryosk".into(),"zlogj".into(),"tyep".into(),"q".into(),"gup".into(),"rf".into(),"j".into()], 3)]
    #[case("tpqojlnhenbzmqkqnxohmzakm", vec!["enbzm".into(),"yy".into(),"xqnjw".into(),"cxwgv".into(),"q".into(),"ras".into(),"ezc".into(),"nt".into(),"eq".into(),"j".into(),"chfw".into(),"v".into(),"ebh".into(),"tvwk".into(),"we".into(),"xhk".into(),"bumlw".into(),"czgy".into(),"njrml".into(),"pl".into(),"cxg".into(),"ztg".into(),"mnvi".into(),"k".into(),"hslr".into(),"fwhrj".into(),"h".into(),"yqro".into(),"vpxyf".into(),"bps".into(),"nhuv".into(),"w".into(),"m".into(),"ln".into(),"nxoh".into(),"skiun".into(),"qnqc".into(),"wtrwp".into(),"hl".into(),"ydbv".into(),"cv".into(),"a".into(),"tpqoj".into(),"umrj".into(),"nq".into(),"xadnx".into(),"emwv".into(),"dmuuw".into()], 1)]
    fn case(#[case] s: String, #[case] dictionary: Vec<String>, #[case] expected: i32) {
        let actual = Solution::min_extra_char(s, dictionary);
        assert_eq!(actual, expected);
    }
}
