//! Solution for https://leetcode.com/problems/find-the-number-of-ways-to-place-people-ii
//! 3027. Find the Number of Ways to Place People II

impl Solution {
    /// Based on https://www.youtube.com/watch?v=8iqtyhTL-U8
    pub fn number_of_pairs(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_by_key(|x| (x[0], -x[1]));
        let mut result = 0;
        for (a_idx, a) in points.iter().enumerate() {
            let mut prev_y = i32::MIN;
            for b in points.iter().skip(a_idx + 1) {
                if b[1] > a[1] {
                    // Too high to be bottom right
                    continue;
                }
                if a[1] >= prev_y && prev_y >= b[1] {
                    // There is a point between
                } else {
                    // Valid pair found
                    result += 1;
                    prev_y = b[1];
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
    #[case(vec![vec![1,1],vec![2,2],vec![3,3]], 0)]
    #[case(vec![vec![6,2],vec![4,4],vec![2,6]], 2)]
    #[case(vec![vec![3,1],vec![1,3],vec![1,1]], 2)]
    fn case(#[case] points: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::number_of_pairs(points);
        assert_eq!(actual, expected);
    }
}
