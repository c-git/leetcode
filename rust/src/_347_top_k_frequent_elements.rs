use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut freq = HashMap::new();
        for num in nums {
            *freq.entry(num).or_insert(0) += 1;
        }

        let mut result: Vec<_> = freq.iter().collect();
        result.sort_unstable_by(|(_, a), (_, b)| b.cmp(a)); // This is fine because the solution is unique
        result.drain(k..);
        result.into_iter().map(|(&k, _)| k).collect()
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    fn validate(mut actual: Vec<i32>, mut expected: Vec<i32>) {
        actual.sort_unstable();
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }

    #[test]
    fn case1() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        let expected = vec![1, 2];
        let actual = Solution::top_k_frequent(nums, k);
        validate(actual, expected);
    }

    #[test]
    fn case2() {
        let nums = vec![1];
        let k = 1;
        let expected = vec![1];
        let actual = Solution::top_k_frequent(nums, k);
        validate(actual, expected);
    }

    #[test]
    fn case3() {
        let nums = vec![1, 1, 1, 1];
        let k = 1;
        let expected = vec![1];
        let actual = Solution::top_k_frequent(nums, k);
        validate(actual, expected);
    }
}
