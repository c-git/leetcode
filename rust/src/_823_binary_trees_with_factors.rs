//! Solution for https://leetcode.com/problems/binary-trees-with-factors
//! 823. Binary Trees With Factors

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn num_factored_binary_trees(mut arr: Vec<i32>) -> i32 {
        // Based on https://leetcode.com/problems/binary-trees-with-factors/solutions/4208932/97-50-optimized-dp/
        arr.sort();
        let s: std::collections::HashSet<_> = arr.iter().cloned().collect();
        let mut dp = std::collections::HashMap::new();
        for &x in &arr {
            dp.insert(x, 1i64);
        }

        for &i in &arr {
            for &j in &arr {
                if (j as i64 * j as i64) > i as i64 {
                    break;
                }
                if i % j == 0 && s.contains(&(i / j)) {
                    let temp = (dp[&j] * dp[&(i / j)]) % MOD;
                    if i / j == j {
                        dp.insert(i, (dp[&i] + temp) % MOD);
                    } else {
                        dp.insert(i, (dp[&i] + temp * 2) % MOD);
                    }
                }
            }
        }

        (dp.values().sum::<i64>() % MOD) as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,4], 3)]
    #[case(vec![2,4,5,10], 7)]
    #[case(vec![2,4,6,8,10,5,3], 16)]
    #[case(vec![2,4,6,8,10,5,3,30], 29)]
    fn case(#[case] arr: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::num_factored_binary_trees(arr);
        assert_eq!(actual, expected);
    }

    #[test]
    fn long() {
        let mut arr = Vec::with_capacity(1000);
        let expected = 320_844;
        (2..).step_by(2).take(1000).for_each(|x| arr.push(x));
        let actual = Solution::num_factored_binary_trees(arr);
        assert_eq!(actual, expected);
    }

    #[test]
    fn big_numbers() {
        let mut arr = Vec::with_capacity(1000);
        let expected = 1000;
        (1_000_000_000 - 2000..)
            .step_by(2)
            .take(1000)
            .for_each(|x| arr.push(x));
        let actual = Solution::num_factored_binary_trees(arr);
        assert_eq!(actual, expected);
    }
}
