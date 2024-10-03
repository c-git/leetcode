//! Solution for https://leetcode.com/problems/binary-search
//! 704. Binary Search

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        Self::search_(nums, target).map(|x| x as i32).unwrap_or(-1)
    }

    // This function only exists to provide more clear semantics (The signature I want not the signature I have)
    fn search_(nums: Vec<i32>, target: i32) -> Option<usize> {
        if &target < nums.first()? {
            // Less than first cannot be in nums
            return None;
        }

        if &target > nums.last()? {
            // Greater than last cannot be in nums
            return None;
        }

        let mut low = 0;
        let mut high = nums.len() - 1;

        // LI: If target is in nums it is in nums[low..=high]
        while low <= high {
            let mid = (low + high) / 2;
            match target.cmp(&nums[mid]) {
                std::cmp::Ordering::Less => high = mid - 1,
                std::cmp::Ordering::Equal => return Some(mid),
                std::cmp::Ordering::Greater => low = mid + 1,
            }
        }
        None
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![-1,0,3,5,9,12], 9, 4)]
    #[case(vec![9,12], 9, 0)]
    #[case(vec![5,9], 9, 1)]
    #[case(vec![-1,0,3,5,9,12], 2, -1)]
    fn case(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: i32) {
        let actual = Solution::search(nums, target);
        assert_eq!(actual, expected);
    }
}
