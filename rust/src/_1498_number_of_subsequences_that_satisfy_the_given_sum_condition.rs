impl Solution {
    pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
        // Implemented based on the editorial
        let (n, mod_) = (nums.len(), 1_000_000_000 + 7);
        nums.sort();
        let mut result = 0;
        let (mut left, mut right) = (0, n - 1);

        while left <= right {
            if nums[left] + nums[right] <= target {
                result = (result + 2i32.pow((right - left) as u32) % mod_) % mod_;
                left += 1;
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
}
