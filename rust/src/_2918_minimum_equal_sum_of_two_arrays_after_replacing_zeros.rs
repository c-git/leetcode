//! Solution for https://leetcode.com/problems/minimum-equal-sum-of-two-arrays-after-replacing-zeros
//! 2918. Minimum Equal Sum of Two Arrays After Replacing Zeros

impl Solution {
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let (has_zeros1, sum1) =
            nums1
                .into_iter()
                .fold((false, 0), |(mut has_zero, mut sum), num| {
                    if num == 0 {
                        has_zero = true;
                        sum += 1;
                    } else {
                        sum += num as i64;
                    }
                    (has_zero, sum)
                });
        let (has_zeros2, sum2) =
            nums2
                .into_iter()
                .fold((false, 0), |(mut has_zero, mut sum), num| {
                    if num == 0 {
                        has_zero = true;
                        sum += 1;
                    } else {
                        sum += num as i64;
                    }
                    (has_zero, sum)
                });

        if sum1 == sum2 {
            return sum1;
        }
        match (has_zeros1, has_zeros2) {
            (true, true) => {}
            (true, false) => {
                if sum2 < sum1 {
                    return -1;
                }
            }
            (false, true) => {
                if sum1 < sum2 {
                    return -1;
                }
            }
            (false, false) => return -1,
        }
        sum1.max(sum2)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![3,2,0,1,0], vec![6,5,0], 12)]
    #[case(vec![2,0,2,0], vec![1,4], -1)]
    fn case(#[case] nums1: Vec<i32>, #[case] nums2: Vec<i32>, #[case] expected: i64) {
        let actual = Solution::min_sum(nums1, nums2);
        assert_eq!(actual, expected);
    }
}
