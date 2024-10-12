//! Solution for https://leetcode.com/problems/k-closest-points-to-origin
//! 973. K Closest Points to Origin

use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
struct PointWrapper(Vec<i32>);

impl PartialOrd for PointWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PointWrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_distance = f64::sqrt((self.0[0] as f64).powi(2) + (self.0[1] as f64).powi(2));
        let other_distance = f64::sqrt((other.0[0] as f64).powi(2) + (other.0[1] as f64).powi(2));
        self_distance
            .partial_cmp(&other_distance)
            .expect("nan is illegal value here")
    }
}

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut result = BinaryHeap::with_capacity(k + 1);
        for point in points {
            result.push(PointWrapper(point));
            if result.len() > k {
                result.pop();
            }
        }
        result.into_iter().map(|x| x.0).collect()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,3],vec![-2,2]], 1, vec![vec![-2,2]])]
    #[case(vec![vec![3,3],vec![5,-1],vec![-2,4]], 2, vec![vec![3,3],vec![-2,4]])]
    fn case(#[case] points: Vec<Vec<i32>>, #[case] k: i32, #[case] mut expected: Vec<Vec<i32>>) {
        let mut actual = Solution::k_closest(points, k);
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }
}
