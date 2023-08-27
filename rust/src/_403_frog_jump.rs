//! Solution for https://leetcode.com/problems/frog-jump
//! 403. Frog Jump

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        debug_assert_eq!(stones[0], 0);
        if stones[1] != 1 {
            return false;
        }
        Self::can_cross_(&stones[1..], 1, 0)
    }

    fn can_cross_(stones: &[i32], jump_size: i32, last_stone: i32) -> bool {
        if stones.is_empty() {
            // You can only get an empty list if you're about to jump off the last stone
            return true;
        }
        let next_middle_stone = last_stone + jump_size;

        if stones.len() == 1 {
            return (next_middle_stone - 1..=next_middle_stone + 1).contains(&stones[0]);
        }

        match stones.binary_search(&next_middle_stone) {
            Ok(middle_idx) => {
                if middle_idx > 0
                    && stones[middle_idx] == next_middle_stone - 1
                    && Self::can_cross_(&stones[middle_idx..], jump_size - 1, next_middle_stone - 1)
                {
                    return true;
                }

                if Self::can_cross_(&stones[middle_idx + 1..], jump_size, next_middle_stone) {
                    return true;
                }

                middle_idx + 1 < stones.len()
                    && stones[middle_idx + 1] == next_middle_stone + 1
                    && Self::can_cross_(
                        &stones[middle_idx + 2..],
                        jump_size + 1,
                        next_middle_stone + 1,
                    )
            }
            Err(after_middle_idx) => {
                if after_middle_idx > 0
                    && stones[after_middle_idx - 1] == next_middle_stone - 1
                    && Self::can_cross_(
                        &stones[after_middle_idx..],
                        jump_size - 1,
                        next_middle_stone - 1,
                    )
                {
                    return true;
                }

                after_middle_idx < stones.len()
                    && stones[after_middle_idx] == next_middle_stone + 1
                    && Self::can_cross_(
                        &stones[after_middle_idx + 1..],
                        jump_size + 1,
                        next_middle_stone + 1,
                    )
            }
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![0,1,3,5,6,8,12,17], true)]
    #[case(vec![0,1,2,3,4,8,9,11], false)]
    #[case(vec![0,2,3,5,6,8,12,17], false)]
    #[case(vec![0,1], true)]
    #[case(vec![0,1,3,6,10,13,14], true)]
    fn case(#[case] stones: Vec<i32>, #[case] expected: bool) {
        let actual = Solution::can_cross(stones);
        assert_eq!(actual, expected);
    }

    #[test]
    fn long() {
        let stones = (0..2000).collect();
        let expected = true;
        case(stones, expected);
    }
}
