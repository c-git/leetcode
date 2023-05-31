impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let result = match nums.binary_search(&target) {
            Ok(x) => x,
            Err(x) => x,
        };
        result as _
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![1,3,5,6], 5, 2)]
    #[case(vec![1,3,5,6], 2, 1)]
    #[case(vec![1,3,5,6], 7, 4)]
    fn case(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: i32) {
        let actual = Solution::search_insert(nums, target);
        assert_eq!(actual, expected);
    }
}
