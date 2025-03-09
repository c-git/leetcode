//! Solution for https://leetcode.com/problems/alternating-groups-ii
//! 3208. Alternating Groups II

use std::collections::VecDeque;

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        let k = k as usize;
        let mut group: VecDeque<i32> = VecDeque::with_capacity(k + 1);
        for color in colors.iter().cycle().take(colors.len() + k - 1) {
            if let Some(last) = group.back() {
                if color == last {
                    // not alternating abandon group
                    group.clear();
                }
            }
            group.push_back(*color);

            if group.len() > k {
                group.pop_front();
            }
            if group.len() == k {
                result += 1;
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
    #[case(vec![0,1,0,1,0], 3, 3)]
    #[case(vec![0,1,0,0,1,0,1], 6, 2)]
    #[case(vec![1,1,0,1], 4, 0)]
    fn case(#[case] colors: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::number_of_alternating_groups(colors, k);
        assert_eq!(actual, expected);
    }
}
