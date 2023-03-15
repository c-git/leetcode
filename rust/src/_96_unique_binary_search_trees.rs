use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        // Used
        // https://leetcode.com/problems/unique-binary-search-trees/solutions/1565543/c-python-5-easy-solutions-w-explanation-optimization-from-brute-force-to-dp-to-catalan-o-n/
        // as a reference (Had base case of 0 wrong)
        let mut memo = HashMap::new();
        Self::num_trees_with_memo(n, &mut memo)
    }
    fn num_trees_with_memo(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        let result = memo.get(&n);
        if let Some(result) = result {
            return *result;
        }
        match n {
            0 | 1 => 1,
            n => {
                let mut result = 0;
                for i in 1..=n {
                    result += Self::num_trees_with_memo(i - 1, memo)
                        * Self::num_trees_with_memo(n - i, memo);
                }
                memo.insert(n, result);
                result
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = 3;
        let expected = 5;

        let actual = Solution::num_trees(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let input = 1;
        let expected = 1;

        let actual = Solution::num_trees(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case3() {
        let input = 19;
        let expected = 1_767_263_190;

        let actual = Solution::num_trees(input);
        assert_eq!(actual, expected);
    }
}
