impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        if digits.is_empty() {
            return vec![];
        }

        let mut carry = 0;
        for i in (0..digits.len()).rev() {
            if digits[i] < 9 {
                digits[i] += 1;
                carry = 0;
                break;
            } else {
                digits[i] = 0;
                carry = 1;
            }
        }

        if carry == 1 {
            digits.insert(0, 1)
        }
        digits
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = vec![1, 2, 3];
        let expected = vec![1, 2, 4];
        let actual = Solution::plus_one(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let input = vec![4, 3, 2, 1];
        let expected = vec![4, 3, 2, 2];
        let actual = Solution::plus_one(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case3() {
        let input = vec![9];
        let expected = vec![1, 0];
        let actual = Solution::plus_one(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case4() {
        let input = vec![9, 9, 9];
        let expected = vec![1, 0, 0, 0];
        let actual = Solution::plus_one(input);
        assert_eq!(actual, expected);
    }
}
