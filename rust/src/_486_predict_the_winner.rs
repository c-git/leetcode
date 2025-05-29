//! Solution for https://leetcode.com/problems/predict-the-winner
//! 486. Predict the Winner

impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        Self::predict_the_winner_(&nums[..], true, 0, 0, sum)
    }

    pub fn predict_the_winner_(
        nums: &[i32],
        is_p1_turn: bool,
        p1_score: i32,
        p2_score: i32,
        sum_left: i32,
    ) -> bool {
        if nums.is_empty() {
            return p1_score >= p2_score;
        }
        if p1_score > sum_left + p2_score {
            return true;
        }
        if is_p1_turn {
            let front = {
                let front = *nums.first().unwrap();
                Self::predict_the_winner_(
                    &nums[1..nums.len()],
                    !is_p1_turn,
                    p1_score + front,
                    p2_score,
                    sum_left - front,
                )
            };
            let back = {
                let back = *nums.last().unwrap();
                Self::predict_the_winner_(
                    &nums[..nums.len() - 1],
                    !is_p1_turn,
                    p1_score + back,
                    p2_score,
                    sum_left - back,
                )
            };
            front || back
        } else {
            let front = {
                let front = *nums.first().unwrap();
                Self::predict_the_winner_(
                    &nums[1..nums.len()],
                    !is_p1_turn,
                    p1_score,
                    p2_score + front,
                    sum_left - front,
                )
            };
            let back = {
                let back = *nums.last().unwrap();
                Self::predict_the_winner_(
                    &nums[..nums.len() - 1],
                    !is_p1_turn,
                    p1_score,
                    p2_score + back,
                    sum_left - back,
                )
            };
            front && back
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
    #[case(vec![1,5,2], false)]
    #[case(vec![1,5,233,7], true)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: bool) {
        let actual = Solution::predict_the_winner(nums);
        assert_eq!(actual, expected);
    }
}
