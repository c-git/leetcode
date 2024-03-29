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
        // Max index occurs when y is equal to the limit as 2 moves required
        // after it passes `y + limit + 1`. Plus an additional 1 for 0 based indexing.
        // Even though this sum is not achievable it doesn't matter as it is guaranteed
        // to be larger than the values to its left and we are only looking for the minimum value
        let mut toggle_edges = vec![0; 2 * limit + 2];
        let mut nums = std::collections::VecDeque::from(nums);
        while let (Some(a), Some(b)) = (nums.pop_front(), nums.pop_back()) {
            let x = a.min(b) as usize;
            let y = a.max(b) as usize;
            // add only edges so total is got by cumulative sum
            toggle_edges[2] += 2;
            toggle_edges[x + 1] -= 1;
            toggle_edges[x + y] -= 1;
            toggle_edges[x + y + 1] += 1;
            toggle_edges[y + limit + 1] += 1;
        }

        let mut min_moves = i32::MAX;
        let mut toggles = toggle_edges.drain(..2).sum();
        for toggle_edge in toggle_edges {
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
