//! Solution for https://leetcode.com/problems/median-of-two-sorted-arrays
//! 4. Median of Two Sorted Arrays

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // Testing lint code solution
        let total = nums1.len() + nums2.len();
        if total % 2 == 1 {
            Self::get_k_th(&nums1, &nums2, total / 2 + 1) as f64
        } else {
            let left = Self::get_k_th(&nums1, &nums2, total / 2);
            let right = Self::get_k_th(&nums1, &nums2, total / 2 + 1);
            (left + right) as f64 / 2f64
        }
    }

    fn get_k_th(a: &[i32], b: &[i32], k: usize) -> i32 {
        println!("a: {a:?}, b: {b:?}, k: {k}");
        // Switch if a is longer than b
        if a.len() > b.len() {
            Self::get_k_th(b, a, k)
        } else if a.is_empty() {
            b[k - 1]
        } else if k == 1 {
            a[0].min(b[0])
        } else {
            let mid1 = a.len().min(k / 2) - 1;
            let mid2 = b.len().min(k / 2) - 1;
            if a[mid1] > b[mid2] {
                Self::get_k_th(a, &b[mid2 + 1..], k - (mid2 + 1))
            } else {
                Self::get_k_th(&a[mid1 + 1..], b, k - (mid1 + 1))
            }
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,3], vec![2], 2.0)]
    #[case(vec![1,2], vec![3,4], 2.5)]
    #[case(vec![1,5,7], vec![2,4,8], 4.5)]
    fn case(#[case] nums1: Vec<i32>, #[case] nums2: Vec<i32>, #[case] expected: f64) {
        let actual = Solution::find_median_sorted_arrays(nums1, nums2);
        assert!((actual - expected).abs() < 1e-5, "Assertion failed: actual {actual:.5} but expected {expected:.5}. Diff is more than 1e-5.");
    }
}
