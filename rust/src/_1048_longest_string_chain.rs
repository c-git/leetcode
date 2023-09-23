//! Solution for https://leetcode.com/problems/longest-string-chain
//! 1048. Longest String Chain

impl Solution {
    pub fn longest_str_chain(mut words: Vec<String>) -> i32 {
        words.sort_unstable_by_key(|x| x.len());
        let mut dp = vec![1; words.len()];
        let mut result = 1;
        for (idx_curr, word) in words.iter().enumerate().skip(1) {
            let curr_len = word.len();
            for idx_prev in (0..idx_curr).rev() {
                match (words[idx_prev].len(), curr_len) {
                    (prev, curr) if prev == curr => continue,
                    (prev, curr) if prev == curr-1 => (),
                    (prev, curr) if prev < curr-1 => break,
                    _ => unreachable!("Should be walking backward through sorted values all cases should be covered"),
                    
                }
                if Self::is_predecessor(&words[idx_prev], word) {
                    let new_chain_len = dp[idx_prev] + 1;
                    if new_chain_len > result {
                        result = new_chain_len;
                    }
                    dp[idx_curr] = dp[idx_curr].max(new_chain_len);
                }
            }
        }
        result
    }

    fn is_predecessor(first: &str, second: &str) -> bool {
        debug_assert_eq!(
            first.len(),
            second.len() - 1,
            "Precondition is first one is one character less than second"
        );
        let first = first.as_bytes();
        let second = second.as_bytes();
        let mut can_skip = true;
        let mut pos_first = 0;
        let mut pos_second = 0;
        while pos_first < first.len() {
            if first[pos_first] != second[pos_second] {
                if can_skip {
                    can_skip = false;
                    pos_second += 1;
                } else {
                    return false;
                }
            } else {
                pos_first += 1;
                pos_second += 1;
            }
        }
        true
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec!["a".into(),"b".into(),"ba".into(),"bca".into(),"bda".into(),"bdca".into()], 4)]
    #[case(vec!["xbc".into(),"pcxbcf".into(),"xb".into(),"cxbc".into(),"pcxbc".into()], 5)]
    #[case(vec!["abcd".into(),"dbqca".into()], 1)]
    #[case(vec!["bdca".into(),"bda".into(),"ca".into(),"dca".into(),"a".into()], 4)]
    fn case(#[case] words: Vec<String>, #[case] expected: i32) {
        let actual = Solution::longest_str_chain(words);
        assert_eq!(actual, expected);
    }
}
