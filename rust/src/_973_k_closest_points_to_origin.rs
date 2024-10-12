//! Solution for https://leetcode.com/problems/k-closest-points-to-origin
//! 973. K Closest Points to Origin

impl Solution {
    pub fn k_closest(mut points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        points.sort_unstable_by(|x, y| {
            let x_distance = f64::sqrt((x[0] as f64).powi(2) + (x[1] as f64).powi(2));
            let y_distance = f64::sqrt((y[0] as f64).powi(2) + (y[1] as f64).powi(2));
            x_distance
                .partial_cmp(&y_distance)
                .expect("nan is illegal value here")
        });
        points.truncate(k as usize);
        points
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
