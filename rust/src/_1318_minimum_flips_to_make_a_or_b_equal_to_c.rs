impl Solution {
    pub fn min_flips(mut a: i32, mut b: i32, mut c: i32) -> i32 {
        let mut result = 0;
        while a | b | c > 0 {
            // use or in while statement to prevent overflow from add

            // Get lowest order bit of each
            let last_bit_a = a % 2;
            let last_bit_b = b % 2;
            let last_bit_c = c % 2;
            match (last_bit_a, last_bit_b, last_bit_c) {
                (0, 0, 0) => {}
                (0, 0, 1) => result += 1,
                (0, 1, 0) => result += 1,
                (0, 1, 1) => {}
                (1, 0, 0) => result += 1,
                (1, 0, 1) => {}
                (1, 1, 0) => result += 2,
                (1, 1, 1) => {}
                _ => {}
            }

            // Right shift all to do next bit
            a >>= 1;
            b >>= 1;
            c >>= 1;
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
    #[case(2, 6, 5, 3)]
    #[case(4, 2, 7, 1)]
    #[case(1, 2, 3, 0)]
    fn case(#[case] a: i32, #[case] b: i32, #[case] c: i32, #[case] expected: i32) {
        let actual = Solution::min_flips(a, b, c);
        assert_eq!(actual, expected);
    }
}
