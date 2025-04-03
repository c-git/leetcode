//! Solution for https://leetcode.com/problems/partition-labels
//! 763. Partition Labels

impl Solution {
    /// After having watched https://www.youtube.com/watch?v=WQZdwxZJAhY
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut result = Vec::new();
        let mut last_seen = vec![0; 26];
        let s = s.as_bytes();
        for (i, &c) in s.iter().enumerate() {
            last_seen[to_index(c)] = i;
        }
        let mut partition_start = 0;
        let mut partition_end = last_seen[to_index(s[0])];
        for (i, &c) in s.iter().enumerate().skip(1) {
            if i > partition_end {
                result.push((partition_end - partition_start + 1) as i32);
                partition_start = i;
            }
            partition_end = partition_end.max(last_seen[to_index(c)]);
        }
        result.push((partition_end - partition_start + 1) as i32);
        result
    }
}

fn to_index(c: u8) -> usize {
    (c - b'a') as usize
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("ababcbacadefegdehijhklij", vec![9,7,8])]
    #[case("eccbbbbdec", vec![10])]
    fn case(#[case] s: String, #[case] expected: Vec<i32>) {
        let actual = Solution::partition_labels(s);
        assert_eq!(actual, expected);
    }
}
