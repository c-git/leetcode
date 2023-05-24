use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct HeapNode {
    freq: i32,
    num: i32,
}

impl Solution {
    // After reading editorial
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut frequencies = HashMap::new();
        for num in nums {
            *frequencies.entry(num).or_insert(0) += 1;
        }

        let mut heap = BinaryHeap::with_capacity(k + 1);
        for (num, freq) in frequencies {
            heap.push(Reverse(HeapNode { freq, num }));
            if heap.len() > k {
                heap.pop();
            }
        }

        heap.into_iter().map(|x| x.0.num).collect()
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    fn validate(mut actual: Vec<i32>, mut expected: Vec<i32>) {
        actual.sort_unstable();
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }

    #[test]
    fn case1() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        let expected = vec![1, 2];
        let actual = Solution::top_k_frequent(nums, k);
        validate(actual, expected);
    }

    #[test]
    fn case2() {
        let nums = vec![1];
        let k = 1;
        let expected = vec![1];
        let actual = Solution::top_k_frequent(nums, k);
        validate(actual, expected);
    }

    #[test]
    fn case3() {
        let nums = vec![1, 1, 1, 1];
        let k = 1;
        let expected = vec![1];
        let actual = Solution::top_k_frequent(nums, k);
        validate(actual, expected);
    }
}
