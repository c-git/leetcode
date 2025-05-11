//! Solution for https://leetcode.com/problems/three-consecutive-odds
//! 1550. Three Consecutive Odds

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        arr.windows(3)
            .any(|window| window.iter().all(|x| *x % 2 == 1))
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,6,4,1], false)]
    #[case(vec![1,2,34,3,4,5,7,23,12], true)]
    fn case(#[case] arr: Vec<i32>, #[case] expected: bool) {
        let actual = Solution::three_consecutive_odds(arr);
        assert_eq!(actual, expected);
    }
}
