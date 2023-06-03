impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut result = "1".to_string();
        for _ in 2..=n {
            let mut temp = "".to_string();
            let mut last_char = result
                .chars()
                .next()
                .expect("Must be at least one character long"); // A char that is not expected
            let mut count = 1;
            for c in result.chars().skip(1) {
                if c == last_char {
                    count += 1;
                } else {
                    temp.push_str(&count.to_string());
                    temp.push(last_char);
                    last_char = c;
                    count = 1;
                }
            }
            temp.push_str(&count.to_string());
            temp.push(last_char);
            result = temp;
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
    #[case(1, "1")]
    #[case(2, "11")]
    #[case(3, "21")]
    #[case(4, "1211")]
    #[case(5, "111221")]
    #[case(6, "312211")]
    fn case(#[case] input: i32, #[case] expected: String) {
        let actual = Solution::count_and_say(input);
        assert_eq!(actual, expected);
    }
}
