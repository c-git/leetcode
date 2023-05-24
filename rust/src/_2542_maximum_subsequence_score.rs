use std::{
    cmp::{max, Reverse},
    collections::BinaryHeap,
};

impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        // Based on solution by sak96
        let mut result = 0i64;

        let k = k as usize;

        // sort by second array
        let mut pairs: Vec<_> = nums1.into_iter().zip(nums2.into_iter()).collect();
        pairs.sort_unstable_by(|(_, a2), (_, b2)| b2.cmp(a2));

        // Get answer for first k elements
        let mut heap = BinaryHeap::with_capacity(k);
        let mut sum = 0i64;
        for (num1, _num2) in pairs.iter().take(k - 1) {
            heap.push(Reverse(*num1));
            sum += *num1 as i64;
        }

        for (num1, num2) in pairs.into_iter().skip(k - 1) {
            sum += num1 as i64;
            heap.push(Reverse(num1));
            result = max(result, sum * num2 as i64);
            sum -= heap.pop().unwrap().0 as i64;
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

    #[test]
    fn case3() {
        let nums1 = vec![2, 1, 14, 12];
        let nums2 = vec![11, 7, 13, 6];
        let k = 3;
        let expected = 168;
        let actual = Solution::max_score(nums1, nums2, k);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case4() {
        let nums1 = vec![
            93, 463, 179, 2488, 619, 2006, 1561, 137, 53, 1765, 2304, 1459, 1768, 450, 1938, 2054,
            466, 331, 670, 1830, 1550, 1534, 2164, 1280, 2277, 2312, 1509, 867, 2223, 1482, 2379,
            1032, 359, 1746, 966, 232, 67, 1203, 2474, 944, 1740, 1775, 1799, 1156, 1982, 1416,
            511, 1167, 1334, 2344,
        ];
        let nums2 = vec![
            345, 229, 976, 2086, 567, 726, 1640, 2451, 1829, 77, 1631, 306, 2032, 2497, 551, 2005,
            2009, 1855, 1685, 729, 2498, 2204, 588, 474, 693, 30, 2051, 1126, 1293, 1378, 1693,
            1995, 2188, 1284, 1414, 1618, 2005, 1005, 1890, 30, 895, 155, 526, 682, 2454, 278, 999,
            1417, 1682, 995,
        ];
        let k = 42;
        let expected = 26653494;
        let actual = Solution::max_score(nums1, nums2, k);
        assert_eq!(actual, expected);
    }
}
