impl Solution {
    pub fn min_bit_flips(mut start: i32, mut goal: i32) -> i32 {
        let mut result = 0;
        while start | goal > 0 {
            // Use or to ensure we don't overflow
            // Check last bit and see if it is the same if not count ad change required
            if (start % 2) ^ (goal % 2) == 1 {
                result += 1;
            }
            start >>= 1;
            goal >>= 1;
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
    #[case(10, 7, 3)]
    #[case(3, 4, 3)]
    fn case(#[case] start: i32, #[case] goal: i32, #[case] expected: i32) {
        let actual = Solution::min_bit_flips(start, goal);
        assert_eq!(actual, expected);
    }
}
