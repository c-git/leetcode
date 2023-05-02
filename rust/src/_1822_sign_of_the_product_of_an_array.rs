impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut result = true;
        for num in &nums {
            match 0.cmp(num) {
                std::cmp::Ordering::Less => (),
                std::cmp::Ordering::Equal => return 0,
                std::cmp::Ordering::Greater => result = !result,
            }
        }

        if result {
            1
        } else {
            -1
        }
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = vec![-1, -2, -3, -4, 3, 2, 1];
        let expected = 1;
        let actual = Solution::array_sign(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let input = vec![1, 5, 0, 2, -3];
        let expected = 0;
        let actual = Solution::array_sign(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case3() {
        let input = vec![-1, 1, -1, 1, -1];
        let expected = -1;
        let actual = Solution::array_sign(input);
        assert_eq!(actual, expected);
    }
}
