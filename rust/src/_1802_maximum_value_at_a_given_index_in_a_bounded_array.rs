

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(,)]
    fn case(#[case] input: , #[case] expected: ) {
        let actual = Solution::(input);
        assert_eq!(actual, expected);
    }
}