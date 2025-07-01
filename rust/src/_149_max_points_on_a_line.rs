//! Solution for https://leetcode.com/problems/max-points-on-a-line
//! 149. Max Points on a Line

use std::collections::HashMap;

fn slope_value(point1: &[i32], point2: &[i32]) -> i64 {
    const DECIMAL_PLACES: f64 = 6.0;
    if point1[0] == point2[0] {
        0
    } else {
        let mut result =
            (point1[1] as f64 - point2[1] as f64) / (point1[0] as f64 - point2[0] as f64);
        result *= DECIMAL_PLACES;
        result as i64
    }
}

impl Solution {
    /// Based on https://www.youtube.com/watch?v=Bb9lOXUOnFw
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let mut result = 1;
        for (first_idx, first_point) in points.iter().enumerate() {
            let mut counts: HashMap<i64, i32> = HashMap::new();
            for second_point in points.iter().skip(first_idx + 1) {
                let slope = slope_value(first_point, second_point);
                let count = counts.entry(slope).or_default();
                *count += 1;
                result = result.max(*count + 1);
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
    #[case(vec![vec![1,1],vec![2,2],vec![3,3]], 3)]
    #[case(vec![vec![1,1],vec![3,2],vec![5,3],vec![4,1],vec![2,3],vec![1,4]], 4)]
    fn case(#[case] points: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::max_points(points);
        assert_eq!(actual, expected);
    }
}
