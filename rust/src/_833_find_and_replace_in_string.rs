//! Solution for https://leetcode.com/problems/find-and-replace-in-string
//! 833. Find And Replace in String

impl Solution {
    pub fn find_replace_string(
        s: String,
        indices: Vec<i32>,
        sources: Vec<String>,
        targets: Vec<String>,
    ) -> String {
        let mut result = String::with_capacity(s.len()); // Will still need to grow in capacity but less times
        let mut sorted_indices: Vec<_> = indices
            .iter()
            .enumerate()
            .filter_map(|(i, x)| {
                if source_matches(*x as usize, &s, &sources[i]) {
                    Some((x, i))
                } else {
                    None
                }
            })
            .collect();
        sorted_indices.sort_unstable();

        let mut copy_from_pos = 0;
        for (_, original_arrays_index) in sorted_indices {
            let replace_start = indices[original_arrays_index] as usize;
            debug_assert!(replace_start >= copy_from_pos);
            if replace_start > copy_from_pos {
                // Copy the difference
                result.push_str(&s[copy_from_pos..replace_start]);
            }
            copy_from_pos = replace_start + sources[original_arrays_index].len(); // Move to next position
            result.push_str(&targets[original_arrays_index]);
        }
        if copy_from_pos < s.len() {
            result.push_str(&s[copy_from_pos..]);
        }

        result
    }
}

fn source_matches(pos: usize, s: &str, test_str: &str) -> bool {
    if s.len() < pos + test_str.len() {
        return false;
    }
    for (original, test) in test_str.chars().zip(s.chars().skip(pos)) {
        if original != test {
            return false;
        }
    }

    true
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("abcd", vec![0, 2], vec!["a".into(), "cd".into()], vec!["eee".into(), "ffff".into()], "eeebffff")]
    #[case("abcd", vec![0, 2], vec!["ab".into(),"ec".into()], vec!["eee".into(),"ffff".into()], "eeecd")]
    fn case(
        #[case] s: String,
        #[case] indices: Vec<i32>,
        #[case] sources: Vec<String>,
        #[case] targets: Vec<String>,
        #[case] expected: String,
    ) {
        let actual = Solution::find_replace_string(s, indices, sources, targets);
        assert_eq!(actual, expected);
    }
}
