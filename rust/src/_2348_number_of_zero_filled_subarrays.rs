impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        todo!()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = vec![1, 3, 0, 0, 2, 0, 0, 4];
        let expected = 6;
        let actual = Solution::zero_filled_subarray(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn case2() {
        let input = vec![0, 0, 0, 2, 0, 0];
        let expected = 9;
        let actual = Solution::zero_filled_subarray(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn case3() {
        let input = vec![2, 10, 2019];
        let expected = 0;
        let actual = Solution::zero_filled_subarray(input);
        assert_eq!(expected, actual);
    }
}
