//! Solution for https://leetcode.com/problems/ugly-number-ii
//! 264. Ugly Number II

use std::cmp::min;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        // Solution from LintCode
        // version 1: O(n) scan
        let n = n as usize;
        let mut uglys = vec![1];
        // p2, p3 & p5 share the same queue: uglys
        let (mut p2, mut p3, mut p5) = (0, 0, 0);
        for i in 1..n {
            let last_number = uglys[i - 1];
            while uglys[p2] * 2 <= last_number {
                p2 += 1;
            }
            while uglys[p3] * 3 <= last_number {
                p3 += 1;
            }
            while uglys[p5] * 5 <= last_number {
                p5 += 1;
            }

            uglys.push(min(uglys[p2] * 2, min(uglys[p3] * 3, uglys[p5] * 5)));
        }
        uglys[n - 1]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(10, 12)]
    #[case(1, 1)]
    fn case(#[case] n: i32, #[case] expected: i32) {
        let actual = Solution::nth_ugly_number(n);
        assert_eq!(actual, expected);
    }
}
