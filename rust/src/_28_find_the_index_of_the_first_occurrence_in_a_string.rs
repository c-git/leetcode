impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if let Some(result) = haystack.find(&needle) {
            result as i32
        } else {
            -1
        }
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("sadbutsad", "sad", 0)]
    #[case("leetcode", "leeto", -1)]
    fn case(#[case] haystack: String, #[case] needle: String, #[case] expected: i32) {
        let actual = Solution::str_str(haystack, needle);
        assert_eq!(actual, expected);
    }
}
