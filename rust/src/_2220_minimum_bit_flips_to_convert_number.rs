impl Solution {
    pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
        // simplified based on fastest submission
        (start ^ goal).count_ones() as i32
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(10, 7, 3)]
    #[case(3, 4, 3)]
    fn case(#[case] start: i32, #[case] goal: i32, #[case] expected: i32) {
        let actual = Solution::min_bit_flips(start, goal);
        assert_eq!(actual, expected);
    }
}
