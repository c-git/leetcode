//! Solution for https://leetcode.com/problems/count-and-say
//! 38. Count and Say

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut result = "1".to_string();
        for _ in 1..n {
            let mut last_char = None;
            let mut run_count = 0;
            let mut temp_result = String::new();
            for c in result.chars() {
                if Some(c) == last_char {
                    run_count += 1;
                } else if let Some(ch) = last_char {
                    temp_result.push_str(&run_count.to_string());
                    temp_result.push(ch);
                    run_count = 1;
                    last_char = Some(c);
                } else {
                    // Was None
                    run_count = 1;
                    last_char = Some(c);
                }
            }

            // Store the last run
            temp_result.push_str(&run_count.to_string());
            temp_result.push(last_char.expect("must end with a character"));
            result = temp_result;
        }
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
    #[case(4, "1211")]
    #[case(1, "1")]
    fn case(#[case] n: i32, #[case] expected: String) {
        let actual = Solution::count_and_say(n);
        assert_eq!(actual, expected);
    }
}
