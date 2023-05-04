use std::mem::swap;
use SenateComposition as sc;

#[derive(Debug)]
enum SenateComposition {
    OnlyRadiant,
    OnlyDire,
    Mixed,
}

impl Solution {
    fn composition_from_first_char(senate: &str) -> SenateComposition {
        match senate.chars().next().unwrap() {
            'R' => sc::OnlyRadiant,
            'D' => sc::OnlyDire,
            _ => unreachable!(),
        }
    }
    pub fn predict_party_victory(mut senate: String) -> String {
        let mut voted_out_r: usize = 0; // Number of r that need to miss a vote
        let mut voted_out_d: usize = 0; // Number of d that need to miss a vote
        let mut next_senate = String::with_capacity(senate.len());
        let mut composition = Self::composition_from_first_char(&senate);
        loop {
            debug_assert!(!senate.is_empty());
            // println!("VotedOut(R, D): ({voted_out_r},{voted_out_d}): Senate: {senate}, Composition: {composition:?}");
            debug_assert!(next_senate.is_empty());
            // Go for a round and let them vote
            for c in senate.chars() {
                match c {
                    'R' => {
                        if voted_out_r == 0 {
                            voted_out_d += 1;
                            next_senate.push(c);
                            composition = Self::update_composition(composition, c);
                        }
                        voted_out_r = voted_out_r.saturating_sub(1);
                    }
                    'D' => {
                        if voted_out_d == 0 {
                            voted_out_r += 1;
                            next_senate.push(c);
                            composition = Self::update_composition(composition, c);
                        }
                        voted_out_d = voted_out_d.saturating_sub(1);
                    }
                    _ => unreachable!("Based on constraints from question on R and D expected"),
                }
                // println!("VotedOut(R, D): ({voted_out_r},{voted_out_d}): Senator: {c}, Composition: {composition:?}");
            }
            swap(&mut senate, &mut next_senate);
            next_senate.clear();
            composition = match composition {
                SenateComposition::OnlyRadiant => return "Radiant".to_string(),
                SenateComposition::OnlyDire => return "Dire".to_string(),
                SenateComposition::Mixed => Self::composition_from_first_char(&senate),
            }
        }
    }

    fn update_composition(composition: SenateComposition, c: char) -> SenateComposition {
        match (composition, c) {
            (sc::Mixed, _) => sc::Mixed,
            (sc::OnlyRadiant, 'R') => sc::OnlyRadiant,
            (sc::OnlyDire, 'D') => sc::OnlyDire,
            (sc::OnlyDire, 'R') => sc::Mixed,
            (sc::OnlyRadiant, 'D') => sc::Mixed,
            _ => unreachable!(),
        }
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = "RD".to_string();
        let expected = "Radiant";
        let actual = Solution::predict_party_victory(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let input = "RDD".to_string();
        let expected = "Dire";
        let actual = Solution::predict_party_victory(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case3() {
        let input = "DDRRR".to_string();
        let expected = "Dire";
        let actual = Solution::predict_party_victory(input);
        assert_eq!(actual, expected);
    }
}
