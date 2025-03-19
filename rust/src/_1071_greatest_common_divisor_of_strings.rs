//! Solution for https://leetcode.com/problems/greatest-common-divisor-of-strings
//! 1071. Greatest Common Divisor of Strings

impl Solution {
    pub fn gcd_of_strings(mut str1: String, mut str2: String) -> String {
        if str1.is_empty() || str2.is_empty() {
            return Default::default();
        }

        // Move shorter string into str1
        if str2.len() < str1.len() {
            std::mem::swap(&mut str1, &mut str2);
        }

        let str1 = str1.as_bytes();
        let str2 = str2.as_bytes();

        let mut result = &str1[..0];

        for (i, c) in str1.iter().copied().enumerate() {
            if str2[i] != c {
                break;
            }
            let prefix = &str1[..=i];
            if str1.len() % prefix.len() == 0
                && str2.len() % prefix.len() == 0
                && str1
                    .windows(prefix.len())
                    .step_by(prefix.len())
                    .all(|win| dbg!(win == prefix))
                && str2
                    .windows(prefix.len())
                    .step_by(prefix.len())
                    .all(|win| dbg!(win == prefix))
            {
                result = &str1[..=i];
            }
        }
        std::str::from_utf8(result)
            .expect("str1 and str2 consist of English uppercase letters.")
            .to_string()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("ABCABC", "ABC", "ABC")]
    #[case("ABABAB", "ABAB", "AB")]
    #[case("LEET", "CODE", "")]
    #[case("ABCABD", "ABC", "")]
    fn case(#[case] str1: String, #[case] str2: String, #[case] expected: String) {
        let actual = Solution::gcd_of_strings(str1, str2);
        assert_eq!(actual, expected);
    }
}
