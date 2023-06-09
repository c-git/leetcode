impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let index = letters.partition_point(|x| x <= &target);
        if index >= letters.len() {
            letters[0]
        } else {
            letters[index]
        }
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec!["c","f","j"], "a", "c")]
    #[case(vec!["c","f","j"], "c", "f")]
    #[case(vec!["x","x","y","y"], "z", "x")]
    #[case(vec!["c","f","j"], "a", "c")]
    fn case(#[case] letters: Vec<&str>, #[case] target: &str, #[case] expected: &str) {
        let letters = letters.iter().map(|x| x.chars().next().unwrap()).collect();
        let target = target.chars().next().unwrap();
        let expected = expected.chars().next().unwrap();
        let actual = Solution::next_greatest_letter(letters, target);
        assert_eq!(actual, expected);
    }
}
