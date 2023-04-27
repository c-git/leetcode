impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
        let start = x;
        let mut converted = 0;
        while x > 0 {
            converted = converted * 10 + x % 10;
            x /= 10;
        }
        converted == start
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = 121;
        let expected = true;
        let actual = Solution::is_palindrome(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let input = -121;
        let expected = false;
        let actual = Solution::is_palindrome(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case3() {
        let input = 10;
        let expected = false;
        let actual = Solution::is_palindrome(input);
        assert_eq!(actual, expected);
    }
}
