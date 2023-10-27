impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        // The cases didn't seem to be able to build upon each other so read the editorial
        // and decided to use the expand from all possible centers solution as it's simple
        // and not slower than the best alternative except for Manacher's Algorithm
        if cfg!(debug_assertions) {
            println!("{s}");
        }
        let s = s.chars().collect::<Vec<char>>();
        let mut start = 0;
        let mut longest = 0;
        for i in 0..s.len() {
            let len1 = Self::expand_around_center(&s[..], i, i);
            let len2 = Self::expand_around_center(&s[..], i, i + 1);
            let len = len1.max(len2);
            if len > longest {
                start = i - (len - 1) / 2;
                longest = len;
            }
        }
        s.iter().skip(start).take(longest).collect()
    }

    fn expand_around_center(s: &[char], left: usize, right: usize) -> usize {
        let mut left = left as i32;
        let mut right = right as i32;
        let n = s.len() as i32;
        while left >= 0 && right < n && s[left as usize] == s[right as usize] {
            left -= 1;
            right += 1;
        }
        (right - left - 1) as usize
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("babad", "bab")]
    #[case("cbbd", "bb")]
    #[case("ccc", "ccc")]
    #[case("adcbaabcdf", "dcbaabcd")]
    #[case("adcbabcdf", "dcbabcd")]
    #[case("dcbabcd", "dcbabcd")]
    #[case("bananas", "anana")]
    fn case(#[case] s: String, #[case] expected: String) {
        let actual = Solution::longest_palindrome(s.clone());
        evaluator(&s, &actual, &expected);
    }

    fn evaluator(input: &str, actual: &str, expected: &str) {
        if actual == expected
            || (actual.len() == expected.len() && input.contains(actual) && is_palindrome(actual))
        {
            // Do nothing test passes
        } else {
            // To trigger failed test
            assert_eq!(actual, expected);
        }
    }

    fn is_palindrome(actual: &str) -> bool {
        let actual: Vec<char> = actual.chars().collect();
        let mut left = 0;
        let mut right = actual.len() - 1;
        while left < right {
            if actual[left] == actual[right] {
                left += 1;
                right -= 1;
            } else {
                return false;
            }
        }
        true
    }
}
