//! Solution for https://leetcode.com/problems/generate-tag-for-video-caption
//! 3582. Generate Tag for Video Caption

impl Solution {
    pub fn generate_tag(caption: String) -> String {
        let mut result = String::from("#");
        let caption = caption.to_lowercase();
        let mut words = caption.split_whitespace();
        if let Some(first_word) = words.next() {
            result.push_str(first_word);
        };
        for word in words {
            result.push_str(
                &word
                    .chars()
                    .next()
                    .expect("assumes at least one character")
                    .to_uppercase()
                    .to_string(),
            );
            if word.len() > 1 {
                result.push_str(&word[1..]);
            }
        }
        result.truncate(100);
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
    #[case("Leetcode daily streak achieved", "#leetcodeDailyStreakAchieved")]
    #[case("can I Go There", "#canIGoThere")]
    #[case("hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh", "#hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh")]
    fn case(#[case] caption: String, #[case] expected: String) {
        let actual = Solution::generate_tag(caption);
        assert_eq!(actual, expected);
    }
}
