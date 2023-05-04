impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut result = 0; // Counts how many R to D so if 0 they are equal and first char wins
        for c in senate.chars() {
            match c {
                'R' => result += 1,
                'D' => result -= 1,
                _ => unreachable!("Based on constraints from question on R and D expected"),
            }
        }

        if result > 0 || (result == 0 && senate.starts_with('R')) {
            "Radiant"
        } else {
            "Dire"
        }
        .to_string()
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
}
