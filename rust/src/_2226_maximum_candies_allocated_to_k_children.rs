//! Solution for https://leetcode.com/problems/maximum-candies-allocated-to-k-children
//! 2226. Maximum Candies Allocated to K Children

impl Solution {
    pub fn maximum_candies(mut candies: Vec<i32>, k: i64) -> i32 {
        let mut result = 0;
        candies.sort_unstable();
        let mut candies_sum = 0;
        for (i, curr_count) in candies.iter().map(|x| *x as i64).enumerate().rev() {
            candies_sum += curr_count;
            let min_clamp_value = if i > 0 {
                if candies[i - 1] < candies[i] {
                    candies[i - 1] + 1
                } else {
                    candies[i]
                }
            } else {
                1
            } as i64;
            let mut per_child = candies_sum / k;
            if per_child == 0 {
                continue;
            }
            for clamp_value in (min_clamp_value..=curr_count).rev() {
                per_child = per_child.min(clamp_value);

                // Test value to see if it can actually work
                let actual_child_count: i64 = candies.iter().map(|x| *x as i64 / per_child).sum();
                if actual_child_count >= k {
                    // This number actually works let's update result
                    result = result.max(per_child);
                    break;
                }
            }
        }
        result as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![5,8,6], 3, 5)]
    #[case(vec![2,5], 11, 0)]
    #[case(vec![5,12,6], 3, 6)]
    #[case(vec![5,60,6], 3, 20)]
    #[case(vec![2,3,4], 3, 2)]
    #[case(vec![2,4,7,5], 4, 3)]
    #[case(vec![4,12], 4, 4)]
    fn case(#[case] candies: Vec<i32>, #[case] k: i64, #[case] expected: i32) {
        let actual = Solution::maximum_candies(candies, k);
        assert_eq!(actual, expected);
    }
}
