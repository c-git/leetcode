//! Solution for https://leetcode.com/problems/longest-palindromic-substring
//! 5. Longest Palindromic Substring

impl Solution {
    #[allow(non_snake_case)]
    pub fn longest_palindrome(s: String) -> String {
        // Implement Manacher's Algorithm based on https://leetcode.com/problems/longest-palindromic-substring/solutions/4212241/98-55-manacher-s-algorithm/
        // See previous commit for simpler solution of expanding around each center
        let mut T = Vec::with_capacity(s.len() * 2 + 3);
        T.push('^');
        T.push('#');
        s.chars().for_each(|c| {
            T.push(c);
            T.push('#')
        });
        T.push('$');

        let n = T.len();
        let mut P = vec![0; n];
        let (mut C, mut R) = (0, 0);

        for i in 1..n - 1 {
            P[i] = if R > i {
                std::cmp::min(R - i, P[2 * C - i])
            } else {
                0
            };
            while T[i + 1 + P[i]] == T[i - 1 - P[i]] {
                P[i] += 1;
            }

            if i + P[i] > R {
                C = i;
                R = i + P[i];
            }
        }

        let (max_len, center_index) = P.iter().enumerate().map(|(i, x)| (x, i)).max().unwrap();
        s[(center_index - max_len) / 2..(center_index + max_len) / 2].into()
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
    #[case("abb", "bb")]
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
