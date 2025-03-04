//! Solution for https://leetcode.com/problems/length-of-longest-fibonacci-subsequence
//! 873. Length of Longest Fibonacci Subsequence

use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct PrevSeq {
    last_value: i32,
    length: i32,
}

impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut prev_sequences = BTreeMap::new();
        prev_sequences.insert(
            arr[0] + arr[1],
            vec![PrevSeq {
                last_value: arr[1],
                length: 2,
            }],
        );
        for (i, &x) in arr.iter().enumerate().skip(2) {
            if let Some(prev_list) = prev_sequences.get(&x).cloned() {
                for prev in prev_list {
                    let next = prev.last_value + x;
                    let candidate = PrevSeq {
                        last_value: x,
                        length: prev.length + 1,
                    };
                    result = result.max(candidate.length);
                    prev_sequences
                        .entry(next)
                        .and_modify(|curr_list| {
                            curr_list.push(candidate);
                        })
                        .or_insert(vec![candidate]);
                }
            }
            for &past_value in arr.iter().take(i) {
                let next = past_value + x;
                let candidate = PrevSeq {
                    last_value: x,
                    length: 2,
                };
                prev_sequences
                    .entry(next)
                    .and_modify(|curr_list| curr_list.push(candidate))
                    .or_insert(vec![candidate]);
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
    #[case(vec![1,2,3,4,5,6,7,8], 5)]
    #[case(vec![1,3,7,11,12,14,18], 3)]
    #[case(vec![2,4,7,8,9,10,14,15,18,23,32,50], 5)]
    fn case(#[case] arr: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::len_longest_fib_subseq(arr);
        assert_eq!(actual, expected);
    }
}
