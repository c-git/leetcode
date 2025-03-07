//! Solution for https://leetcode.com/problems/closest-prime-numbers-in-range
//! 2523. Closest Prime Numbers in Range

impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let left = left as usize;
        let right = right as usize;
        let mut result = vec![-1, -1];
        let mut best_gap = usize::MAX;
        let mut last_prime: Option<usize> = None;
        // True = prime (or not checked yet)
        let mut numbers = vec![true; right + 1];
        for i in 2..=right {
            if numbers[i] {
                // Is prime
                if left <= i && i <= right {
                    if let Some(prev) = last_prime {
                        let curr_gap = i - prev;
                        if curr_gap < best_gap {
                            best_gap = curr_gap;
                            result[0] = prev as _;
                            result[1] = i as _;
                        }
                    }
                    last_prime = Some(i);
                }
                mark_multiples_not_prime(i, &mut numbers);
            }
        }

        result
    }
}

fn mark_multiples_not_prime(base: usize, numbers: &mut [bool]) {
    for i in (base..).skip(base).step_by(base) {
        if i >= numbers.len() {
            break;
        }
        numbers[i] = false;
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(10, 19, vec![11,13])]
    #[case(4, 6, vec![-1,-1])]
    fn case(#[case] left: i32, #[case] right: i32, #[case] expected: Vec<i32>) {
        let actual = Solution::closest_primes(left, right);
        assert_eq!(actual, expected);
    }
}
