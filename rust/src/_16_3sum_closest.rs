impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let mut result: i32 = nums.iter().take(3).sum();
        let mut best_diff = (target - result).abs();
        for (index1, num1) in nums.iter().enumerate() {
            for (index2, num2) in nums.iter().enumerate() {
                if index1 != index2 {
                    let partial = num1 + num2;
                    let remainder = target - partial;
                    let second_partition_start = nums.partition_point(|x| x < &remainder);

                    if second_partition_start > 0 {
                        let first_partition_end = second_partition_start - 1;
                        if first_partition_end != index1 && first_partition_end != index2 {
                            let attempt = partial + nums[first_partition_end];
                            Self::compare_result(target, attempt, &mut best_diff, &mut result);
                        }
                    }
                    if second_partition_start < nums.len()
                        && second_partition_start != index1
                        && second_partition_start != index2
                    {
                        let attempt = partial + nums[second_partition_start];
                        Self::compare_result(target, attempt, &mut best_diff, &mut result);
                    }
                    if best_diff == 0 {
                        return result;
                    }
                }
            }
        }
        result
    }

    #[inline]
    fn compare_result(target: i32, attempt: i32, best_diff: &mut i32, result: &mut i32) {
        let diff = (target - attempt).abs();
        if diff < *best_diff {
            *result = attempt;
            *best_diff = diff;
        }
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
