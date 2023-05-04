impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        debug_assert!(
            num >= 1,
            "What the romans didn't have 0 or negative numbers"
        );
        let mut result = String::new();
        let values = [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];
        while num > 0 {
            for &(value, symbol) in &values {
                if num >= value {
                    num -= value;
                    result.push_str(symbol);
                    break;
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

    #[test]
    fn case1() {
        let input = 3;
        let expected = "III";
        let actual = Solution::int_to_roman(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let input = 58;
        let expected = "LVIII";
        let actual = Solution::int_to_roman(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case3() {
        let input = 1994;
        let expected = "MCMXCIV";
        let actual = Solution::int_to_roman(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case4() {
        let input = 3999;
        let expected = "MMMCMXCIX";
        let actual = Solution::int_to_roman(input);
        assert_eq!(actual, expected);
    }
}
