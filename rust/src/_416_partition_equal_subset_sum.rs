//! Solution for https://leetcode.com/problems/partition-equal-subset-sum
//! 416. Partition Equal Subset Sum

impl Solution {
    // Based on https://www.youtube.com/watch?v=snue4L5WrJ4
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let mut possible_sums = std::collections::BTreeSet::new();
        possible_sums.insert(0); // We can always get 0 by adding none
        let sum: i32 = nums.iter().sum();
        if sum % 2 == 1 {
            return false;
        }
        let target = sum / 2;
        for num in nums {
            let mut upper = *possible_sums.last().unwrap() + 1;
            while let Some(&next) = possible_sums.range(0..upper).next_back() {
                upper = next;
                let new_possible_sum = num + next;
                match new_possible_sum.cmp(&target) {
                    std::cmp::Ordering::Equal => return true,
                    std::cmp::Ordering::Greater => (), // Discard as this is too big
                    std::cmp::Ordering::Less => {
                        possible_sums.insert(new_possible_sum);
                    }
                }
            }
        }

        false
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,5,11,5], true)]
    #[case(vec![1,2,3,5], false)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: bool) {
        let actual = Solution::can_partition(nums);
        assert_eq!(actual, expected);
    }
}
