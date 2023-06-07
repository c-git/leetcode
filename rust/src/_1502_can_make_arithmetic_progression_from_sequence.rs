impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort_unstable();
        let diff = arr[1] - arr[0];
        let mut last = arr[1];
        for curr in arr.iter().skip(2) {
            if curr - last != diff {
                return false;
            }
            last = *curr;
        }
        true
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![3,5,1], true)]
    #[case(vec![1,2,4], false)]
    fn case(#[case] input: Vec<i32>, #[case] expected: bool) {
        let actual = Solution::can_make_arithmetic_progression(input);
        assert_eq!(actual, expected);
    }
}
