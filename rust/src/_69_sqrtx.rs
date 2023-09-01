//! Solution for https://leetcode.com/problems/sqrtx
//! 69. Sqrt(x)

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut lower = 0;
        let mut upper = x / 2;
        while upper * upper > x {
            let mid = (lower + upper) / 2;
            if mid * mid > x {
                upper = mid;
            } else {
                if lower == mid {
                    return lower;
                }
                lower = mid;
            }
        }
        upper
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(4, 2)]
    #[case(8, 2)]
    #[case(0, 0)]
    #[case(50, 7)]
    #[case(500, 22)]
    #[case(625, 25)]
    fn case(#[case] x: i32, #[case] expected: i32) {
        let actual = Solution::my_sqrt(x);
        assert_eq!(actual, expected);
    }
}
