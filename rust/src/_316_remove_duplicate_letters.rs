//! Solution for https://leetcode.com/problems/remove-duplicate-letters
//! 316. Remove Duplicate Letters

use std::collections::{HashMap, HashSet};

impl Solution {
    // Taken from https://leetcode.com/problems/remove-duplicate-letters/solutions/4090711/98-53-stack-and-greedy/
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut stack: Vec<char> = Vec::new();
        let mut seen: HashSet<char> = HashSet::new();
        let mut last_occ: HashMap<char, usize> = HashMap::new();
        for (i, c) in s.chars().enumerate() {
            last_occ.insert(c, i);
        }

        for (i, c) in s.chars().enumerate() {
            if !seen.contains(&c) {
                while let Some(&top) = stack.last() {
                    if c < top && i < *last_occ.get(&top).unwrap() {
                        seen.remove(&stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                seen.insert(c);
                stack.push(c);
            }
        }

        stack.into_iter().collect()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("bcabc", "abc")]
    #[case("cbacdcbc", "acdb")]
    fn case(#[case] s: String, #[case] expected: String) {
        let actual = Solution::remove_duplicate_letters(s);
        assert_eq!(actual, expected);
    }
}
