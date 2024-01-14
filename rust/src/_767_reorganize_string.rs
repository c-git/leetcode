//! Solution for https://leetcode.com/problems/reorganize-string
//! 767. Reorganize String

use std::collections::HashMap;

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let mut result = String::with_capacity(s.len());
        let mut freq: HashMap<char, usize> = HashMap::new();
        s.chars().for_each(|c| *freq.entry(c).or_default() += 1);
        let mut remaining: Vec<(usize, char)> = freq.into_iter().map(|(k, v)| (v, k)).collect();
        remaining.sort_unstable();
        let mut most_freq = remaining.pop();
        while let Some((mut next_count, next_c)) = remaining.pop() {
            let (mut most_freq_count, most_freq_c) = most_freq.expect(
                "This should always be some at the top of the loop otherwise we've made a mistake",
            );
            debug_assert!(
                most_freq_count > 0 && next_count > 0,
                "Values assumed to be greater than 0 to start"
            );
            while most_freq_count > 1 && next_count > 0 {
                result.push(most_freq_c);
                result.push(next_c);
                most_freq_count -= 1;
                next_count -= 1;
            }
            if next_count > 0 {
                debug_assert_eq!(
                    most_freq_count, 1,
                    "Loop exited and next still has left so this one is done we can use it now"
                );
                result.push(most_freq_c);
                most_freq_count -= 1; // Bring it to 0 makes the logic below easier to follow
            }
            match (most_freq_count, next_count) {
                (0, 0) => most_freq = remaining.pop(),
                (0, x) => {
                    // The most_freq ran out
                    debug_assert!(
                        x > 0,
                        "Because it's a usize and not equal to 0 so must be greater than 0"
                    );
                    most_freq = Some((x, next_c));
                }
                (x, 0) => {
                    debug_assert!(
                        x > 0,
                        "Because it's a usize and not equal to 0 so must be greater than 0"
                    );
                    most_freq = Some((x, most_freq_c));
                }
                _ => unreachable!("One of them must be 0 to exit the loop"),
            }
        }

        match most_freq {
            Some((1, c)) => {
                result.push(c);
                result
            }
            None => result,
            Some(_) => "".to_string(), // More than one left cannot work
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("aab", "aba")]
    #[case("aaab", "")]
    #[case("zhmyo", "zyomh")]
    #[case("ogccckcwmbmxtsbmozli", "cocgcickmlmsmtbwbxoz")]
    fn case(#[case] s: String, #[case] expected: String) {
        let actual = Solution::reorganize_string(s);
        assert_eq!(actual.len(), expected.len(), "Length should be the same");
        check_no_repeats(actual);
    }

    fn check_no_repeats(actual: String) {
        let Some(mut last_char) = actual.chars().next() else {
            return;
        };
        for c in actual.chars().skip(1) {
            assert_ne!(c, last_char, "Duplicate character found in {actual}");
            last_char = c;
        }
    }
}
