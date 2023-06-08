use std::collections::BTreeMap;

impl Solution {
    /// Source: sak96 - https://leetcode.com/problems/minimum-moves-to-make-array-complementary/solutions/3611937/toggle-distribution/
    ///
    /// Special thanks to: https://github.com/vim-scripts/DrawIt
    ///
    /// Toggle distribution based for pair (x,y) with x < y
    /// Based on final sum that is chosen
    ///
    ///                   [no toggle]
    ///   [reduce]            0                [increase]
    ///   [x & y ] [reduce y] | [increase x]   [ x & y  ]
    ///   +-------+           v               +-----------+
    ///   |       |-----------+---------------|           |
    ///   |   2   |    1      |      1        |     2     |
    ///   +-------+-----------+---------------+-----------+
    ///  2       x           x+y     y + limit
    ///  ----------------- final sum ---------------------->
    ///
    pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        let limit = limit as usize;
        let mut toggle_edges = BTreeMap::new();
        let mut nums = std::collections::VecDeque::from(nums);
        while let (Some(a), Some(b)) = (nums.pop_front(), nums.pop_back()) {
            let x = a.min(b) as usize;
            let y = a.max(b) as usize;
            // add only edges so total is got by cumulative sum
            *toggle_edges.entry(2).or_default() += 2;
            *toggle_edges.entry(x + 1).or_default() -= 1;
            *toggle_edges.entry(x + y).or_default() -= 1;
            *toggle_edges.entry(x + y + 1).or_default() += 1;
            *toggle_edges.entry(y + limit + 1).or_default() += 1;
        }

        let mut min_moves = i32::MAX;
        let mut toggles = 0;
        for (_sum, toggle_edge) in toggle_edges.iter() {
            toggles += toggle_edge;
            min_moves = min_moves.min(toggles);
        }
        min_moves
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,4,3], 4, 1)]
    #[case(vec![1,2,2,1], 2, 2)]
    #[case(vec![1,2,1,2], 2, 0)]
    #[case(vec![1,1,2,2,1,1], 2, 2)]
    #[case(vec![20744,7642,19090,9992,2457,16848,3458,15721], 22891, 4)]
    #[case(vec![207,76,190,99,24,168,34,157], 228, 4)]
    fn case(#[case] nums: Vec<i32>, #[case] limit: i32, #[case] expected: i32) {
        let actual = Solution::min_moves(nums, limit);
        assert_eq!(actual, expected);
    }
}
