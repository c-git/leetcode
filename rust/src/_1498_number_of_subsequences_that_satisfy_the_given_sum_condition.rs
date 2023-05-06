impl Solution {
    pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        // Implemented based on the editorial
        let (n, mod_) = (nums.len(), 1_000_000_000 + 7);
        nums.sort();
        let mut result = 0;
        let (mut left, mut right) = (0, n - 1);

        // Precompute the value of 2 to the power of each value.
        let mut powers: Vec<i32> = Vec::with_capacity(n);
        powers.push(1); //powers[0] = 1;
        for i in 1..n {
            powers.push((powers[i - 1] * 2) % mod_); //powers[i] =
        }

        while left <= right {
            if nums[left] + nums[right] <= target {
                result = (result + powers[right - left]) % mod_;
                left += 1;
            } else if right == 0 {
                break;
            } else {
                right -= 1;
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
        let nums = vec![3, 5, 6, 7];
        let target = 9;
        let expected = 4;
        let actual = Solution::num_subseq(nums, target);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let nums = vec![3, 3, 6, 8];
        let target = 10;
        let expected = 6;
        let actual = Solution::num_subseq(nums, target);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case3() {
        let nums = vec![2, 3, 3, 4, 6, 7];
        let target = 12;
        let expected = 61;
        let actual = Solution::num_subseq(nums, target);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case4() {
        let nums = vec![1];
        let target = 1;
        let expected = 0;
        let actual = Solution::num_subseq(nums, target);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case5() {
        let nums = vec![
            14, 4, 6, 6, 20, 8, 5, 6, 8, 12, 6, 10, 14, 9, 17, 16, 9, 7, 14, 11, 14, 15, 13, 11,
            10, 18, 13, 17, 17, 14, 17, 7, 9, 5, 10, 13, 8, 5, 18, 20, 7, 5, 5, 15, 19, 14,
        ];
        let target = 22;
        let expected = 272187084;
        let actual = Solution::num_subseq(nums, target);
        assert_eq!(actual, expected);
    }
}
