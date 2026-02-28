//! Solution for https://leetcode.com/problems/last-stone-weight-ii
//! 1049. Last Stone Weight II

impl Solution {
    /// Based on https://www.youtube.com/watch?v=gdXkkmzvR3c
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let stone_sum: i32 = stones.iter().sum();
        let target = (stone_sum + 1) / 2;
        let mut memo: Vec<Vec<Option<i32>>> = vec![vec![None; stones.len()]; stone_sum as usize];
        Self::last_stone_weight_ii_(0, 0, &stones, stone_sum, target, &mut memo)
    }

    fn last_stone_weight_ii_(
        i: usize,
        total: i32,
        stones: &[i32],
        stone_sum: i32,
        target: i32,
        memo: &mut [Vec<Option<i32>>],
    ) -> i32 {
        if total >= target || i == stones.len() {
            return (total - (stone_sum - total)).abs();
        }
        if let Some(result) = memo[total as usize][i] {
            return result;
        }
        let result = std::cmp::min(
            Self::last_stone_weight_ii_(i + 1, total, stones, stone_sum, target, memo),
            Self::last_stone_weight_ii_(i + 1, total + stones[i], stones, stone_sum, target, memo),
        );
        memo[total as usize][i] = Some(result);
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
    #[case(vec![2,7,4,1,8,1], 1)]
    #[case(vec![31,26,33,21,40], 5)]
    fn case(#[case] stones: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::last_stone_weight_ii(stones);
        assert_eq!(actual, expected);
    }
}
