//! Solution for https://leetcode.com/problems/alternating-groups-ii
//! 3208. Alternating Groups II

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        let k = k as usize;
        let mut last = colors[0];
        let mut group_start = 0;
        for (i, color) in colors
            .iter()
            .copied()
            .cycle()
            .take(colors.len() + k - 1)
            .enumerate()
        {
            if color == last {
                // not alternating abandon group
                group_start = i;
            }
            last = color;

            if i - group_start >= k {
                group_start += 1;
            }
            if i - group_start == k - 1 {
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
