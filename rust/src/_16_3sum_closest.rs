impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        // Updated based on fastest submission
        nums.sort_unstable();
        let n = nums.len();
        let mut result: i32 = nums.iter().take(3).sum();
        let mut best_diff = (target - result).abs();
        for (index1, &num1) in nums.iter().take(n - 2).enumerate() {
            if index1 > 0 && nums[index1 - 1] == num1 {
                continue;
            }
            // set left and right sides of search
            let (mut index2, mut index3) = (index1 + 1, n - 1);

            let inner_target = num1 - target;
            while index2 < index3 {
                match inner_target + nums[index2] + nums[index3] {
                    0 => return target,
                    x if x < 0 => {
                        if x.abs() < best_diff {
                            result = target + x;
                            best_diff = x.abs();
                        }
                        index2 += 1;
                    }
                    x => {
                        if x.abs() < best_diff.abs() {
                            result = target + x;
                            best_diff = x.abs();
                        }
                        index3 -= 1;
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
    use rstest::rstest;

    #[rstest]
    #[case(vec![-1,2,1,-4], 1, 2)]
    #[case(vec![0,0,0], 1, 0)]
    fn case(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: i32) {
        let actual = Solution::three_sum_closest(nums, target);
        assert_eq!(actual, expected);
    }
}
