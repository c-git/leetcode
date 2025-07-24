//! Solution for https://leetcode.com/problems/hand-of-straights
//! 846. Hand of Straights

use std::collections::BTreeMap;

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        if hand.len() % group_size as usize != 0 {
            // Not divisible by group size
            return false;
        }

        let mut cards = BTreeMap::new();
        for card in hand {
            *cards.entry(card).or_default() += 1;
        }

        while !cards.is_empty() {
            let start_value = *cards.first_entry().unwrap().key();
            for next_required_key in start_value..(start_value + group_size) {
                match cards.get_mut(&next_required_key) {
                    None => return false, // needed but not found
                    Some(1) => {
                        // Last value remove key from map
                        cards.remove(&next_required_key);
                    }
                    Some(freq_remaining) => {
                        // Reduce remaining by 1
                        *freq_remaining -= 1;
                    }
                }
            }
        }

        true
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3,6,2,3,4,7,8], 3, true)]
    #[case(vec![1,2,3,4,5], 4, false)]
    fn case(#[case] hand: Vec<i32>, #[case] group_size: i32, #[case] expected: bool) {
        let actual = Solution::is_n_straight_hand(hand, group_size);
        assert_eq!(actual, expected);
    }
}
