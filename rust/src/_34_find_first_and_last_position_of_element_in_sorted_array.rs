impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        Self::_search_range(nums, target).unwrap_or_else(|| vec![-1; 2])
    }

    fn _search_range(nums: Vec<i32>, target: i32) -> Option<Vec<i32>> {
        let n = nums.len();
        let left = nums.partition_point(|&x| x < target);
        if left == n || nums[left] != target {
            return None;
        }
        let right = nums.partition_point(|&x| x <= target) - 1;

        Some(vec![left as i32, right as i32])
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![5,7,7,8,8,10], 8, vec![3,4])]
    #[case(vec![5,7,7,8,8,10], 6, vec![-1,-1])]
    #[case(vec![], 0, vec![-1,-1])]
    fn case(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: Vec<i32>) {
        let actual = Solution::search_range(nums, target);
        assert_eq!(actual, expected);
    }
}
