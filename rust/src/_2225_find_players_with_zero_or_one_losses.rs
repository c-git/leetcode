//! Solution for https://leetcode.com/problems/find-players-with-zero-or-one-losses
//! 2225. Find Players With Zero or One Losses

use std::collections::BTreeSet;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut winners = BTreeSet::new();
        let mut losers = BTreeSet::new();
        let mut seen = BTreeSet::new();

        for outcome in matches {
            let winner = outcome[0];
            let loser = outcome[1];

            // Handle winner
            if !seen.contains(&winner) {
                debug_assert!(!winners.contains(&winner));
                winners.insert(winner);
                seen.insert(winner);
            }

            // Handle loser
            if !seen.contains(&loser) {
                debug_assert!(!losers.contains(&loser));
                losers.insert(loser);
                seen.insert(loser);
            } else if winners.remove(&loser) {
                losers.insert(loser);
            } else {
                losers.remove(&loser);
            }
        }

        vec![winners.into_iter().collect(), losers.into_iter().collect()]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,3],vec![2,3],vec![3,6],vec![5,6],vec![5,7],vec![4,5],vec![4,8],vec![4,9],vec![10,4],vec![10,9]], vec![vec![1,2,10],vec![4,5,7,8]])]
    #[case(vec![vec![2,3],vec![1,3],vec![5,4],vec![6,4]], vec![vec![1,2,5,6],vec![]])]
    fn case(#[case] matches: Vec<Vec<i32>>, #[case] expected: Vec<Vec<i32>>) {
        let actual = Solution::find_winners(matches);
        assert_eq!(actual, expected);
    }
}
