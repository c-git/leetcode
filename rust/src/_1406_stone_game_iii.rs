use std::cmp::{min, Ordering::*};

#[derive(Clone)]
struct BestScore {
    score: i32,
    taken: Option<u8>,
}

impl BestScore {
    fn new(score: i32, taken: u8) -> Self {
        Self {
            score,
            taken: Some(taken),
        }
    }
    fn min_value() -> Self {
        Self {
            score: i32::MIN,
            taken: None,
        }
    }

    fn taken_usize(&self) -> usize {
        self.taken.unwrap_or_default() as usize
    }
}
impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let n = stone_value.len();

        // Stores the best score achievable starting at that position in the stone values
        let mut dp = vec![BestScore::min_value(); n + 1]; // Extra one value to be set to 0 to avoid check of pass end of list

        // Set base case
        dp[n].score = 0;

        // Set other cases
        for i in (0..n).rev() {
            // Uses value it was initialized to as a min value be updated so must be initialized to min
            let mut best_diff = i32::MIN;
            for take_end in i..min(n, i + 3) {
                let other_score_index = take_end + 1;
                let other_score = dp[other_score_index].score;
                let next_start_index = other_score_index + dp[other_score_index].taken_usize();
                let score_after_this_take = dp[next_start_index].score;
                let this_score =
                    stone_value[i..=take_end].iter().sum::<i32>() + score_after_this_take;
                let this_diff = this_score - other_score;
                if this_diff > best_diff {
                    best_diff = this_diff;
                    dp[i].score = this_score;
                    dp[i].taken = Some((take_end - i + 1) as u8);
                }
            }
        }

        let alice_score = dp[0].score;
        let bob_score_index = dp[0].taken_usize();
        let bob_score = dp[bob_score_index].score;
        match alice_score.cmp(&bob_score) {
            Less => "Bob",
            Equal => "Tie",
            Greater => "Alice",
        }
        .to_string()
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3,7],"Bob")]
    #[case(vec![1,2,3,-9],"Alice")]
    #[case(vec![1,2,3,6],"Tie")]
    #[case(vec![5,3,-2,8,9,-2,4,8,10,20],"Alice")]
    #[case(vec![1,2,-5,3,4,-8,10,-4,15,-8,5,0,1],"Alice")]
    #[case(vec![1],"Alice")]
    #[case(vec![-1],"Bob")]
    fn case(#[case] input: Vec<i32>, #[case] expected: &str) {
        let actual = Solution::stone_game_iii(input);
        assert_eq!(actual, expected);
    }
}
