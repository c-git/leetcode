use std::{cmp::max, collections::BinaryHeap};

impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        // Based on solution by sak96
        let mut result = 0i64;

        let k = k as usize;

        // sort by second array
        let mut pairs: Vec<_> = nums1.into_iter().zip(nums2).collect();
        pairs.sort_unstable_by(|(_, a2), (_, b2)| b2.cmp(a2));

        // Get answer for first k elements
        let mut heap = BinaryHeap::with_capacity(k);
        let mut sum = 0i64;
        for (num1, _num2) in pairs.iter().take(k - 1) {
            heap.push(*num1);
            sum += *num1 as i64;
        }

        for (num1, num2) in pairs.into_iter().skip(k - 1) {
            sum += num1 as i64;
            heap.push(num1);
            result = max(result, sum * num2 as i64);
            sum -= heap.pop().unwrap() as i64;
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
        let nums1 = vec![1, 3, 3, 2];
        let nums2 = vec![2, 1, 3, 4];
        let k = 3;
        let expected = 12;
        let actual = Solution::max_score(nums1, nums2, k);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let nums1 = vec![4, 2, 3, 1, 1];
        let nums2 = vec![7, 5, 10, 9, 6];
        let k = 1;
        let expected = 30;
        let actual = Solution::max_score(nums1, nums2, k);
        assert_eq!(actual, expected);
    }
}
