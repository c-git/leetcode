impl Solution {
    // After reading editorial
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = vec![];
        if n == 0 {
            return vec!["".to_string()];
        }
        for left_count in 0..n {
            for left_string in Self::generate_parenthesis(left_count) {
                for right_string in Self::generate_parenthesis(n - 1 - left_count) {
                    result.push(format!("({left_string}){right_string}"));
                }
            }
        }

        result
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1, vec!["()"])]
    #[case(3, vec!["((()))","(()())","(())()","()(())","()()()"])]
    fn case(#[case] input: i32, #[case] mut expected: Vec<&str>) {
        let mut actual = Solution::generate_parenthesis(input);
        actual.sort_unstable();
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }
}
