impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let mapping = [
            ("M", 1000),
            ("CM", 900),
            ("D", 500),
            ("CD", 400),
            ("C", 100),
            ("XC", 90),
            ("L", 50),
            ("XL", 40),
            ("X", 10),
            ("IX", 9),
            ("V", 5),
            ("IV", 4),
            ("I", 1),
        ];
        let mut s = &s[..];
        for (symbol, value) in mapping {
            while s.starts_with(symbol) {
                result += value;
                s = &s[symbol.len()..];
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
        let input = "III".into();
        let expected = 3;
        let actual = Solution::roman_to_int(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let input = "LVIII".into();
        let expected = 58;
        let actual = Solution::roman_to_int(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case3() {
        let input = "MCMXCIV".into();
        let expected = 1994;
        let actual = Solution::roman_to_int(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case4() {
        let input = "MMMCMXCIX".into();
        let expected = 3999;
        let actual = Solution::roman_to_int(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case5() {
        let input = "CDXLIV".into();
        let expected = 444;
        let actual = Solution::roman_to_int(input);
        assert_eq!(actual, expected);
    }
}
