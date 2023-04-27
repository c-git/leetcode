impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        // Pattern discovered is that only number with an odd number of factors will be on.
        // Because each number basically performs and xor with the previous numbers so the bulbs ("bits") will get flipped each time
        // and only odd number number of factors will result in the light staying on. Only square numbers have an odd number of factors
        // because one factor is repeated and that factor is the square root.

        let mut result = 0;

        // Count square numbers less than n
        for x in 1..=n {
            if x * x <= n {
                result += 1;
            } else {
                break;
            }
        }
        result
    }
}
struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = 3;
        let expected = 1;
        let actual = Solution::bulb_switch(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let input = 0;
        let expected = 0;
        let actual = Solution::bulb_switch(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case3() {
        let input = 1;
        let expected = 1;
        let actual = Solution::bulb_switch(input);
        assert_eq!(actual, expected);
    }
}
