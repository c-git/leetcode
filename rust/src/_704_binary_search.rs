//! Solution for https://leetcode.com/problems/binary-search
//! 704. Binary Search

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let index = partition_point(&nums, |x| x < &target);
        if let Some(num) = nums.get(index) {
            if num == &target {
                index as _
            } else {
                -1
            }
        } else {
            -1
        }
    }
}

/// Returns the index of the partition point according to the given predicate (the index of the first element of the second partition).
/// See <https://doc.rust-lang.org/std/primitive.slice.html#method.partition_point> for more info
fn partition_point<T>(arr: &[T], is_left_half: impl Fn(&T) -> bool) -> usize {
    let mut size = arr.len();
    if size == 0 {
        return 0;
    }
    let mut base = 0usize;
    while size > 1 {
        let half = size / 2;
        let mid = base + half;
        base = if is_left_half(&arr[mid]) { mid } else { base };
        size -= half;
    }
    if is_left_half(&arr[base]) {
        base + 1
    } else {
        base
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
