//! Solution for https://leetcode.com/problems/minimum-operations-to-exceed-threshold-value-ii
//! 3066. Minimum Operations to Exceed Threshold Value II

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>, k: i32) -> i32 {
        // ASSUMPTION: Based on the guarantee from the question the unwraps below are valid
        // - The input is generated such that an answer always exists
        nums.iter_mut().for_each(|x| *x *= -1);
        let mut result = 0;
        let mut heap = std::collections::BinaryHeap::from(nums);
        while (heap.peek().unwrap() * -1) < k {
            result += 1;
            let x = heap.pop().unwrap();
            let y = heap.pop().unwrap();
            heap.push(x * 2 + y);
        }
        result
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,11,10,1,3], 10, 2)]
    #[case(vec![1,1,2,4,9], 20, 4)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::min_operations(nums, k);
        assert_eq!(actual, expected);
    }
}
