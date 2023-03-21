impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut result = 0;
        let mut on_zero_subarray = false;
        let mut curr_len = 0;
        for num in nums {
            match num {
                0 => {
                    if !on_zero_subarray {
                        on_zero_subarray = true;
                        curr_len = 0;
                    }
                    curr_len += 1;
                    result += curr_len;
                }
                _ => {
                    if on_zero_subarray {
                        on_zero_subarray = false;
                    }
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
