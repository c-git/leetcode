use std::{cmp::Reverse, collections::BinaryHeap};

struct KthLargest {
    k: usize,
    k_top: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    // Updated based on faster submissions
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let k = k as usize;
        debug_assert!(nums.len() >= k - 1);
        let mut result = Self {
            k,
            k_top: BinaryHeap::with_capacity(k + 1),
        };
        for num in nums {
            result.add(num);
        }
        result
    }

    /// Add the new value and returns smallest value (only up to k values saved)
    ///
    /// If less than k values are stored the the value returned is not the kth largest
    /// This is fine because of the constraint in the question that it will only check
    /// the answer after k items have been added
    fn add(&mut self, val: i32) -> i32 {
        self.k_top.push(Reverse(val));
        if self.k_top.len() > self.k {
            // Only needs to be done once as at most 1 item can be added
            self.k_top.pop(); // Discard the smallest value
        }
        debug_assert!(self.k_top.len() <= self.k);

        // Top of heap will always be the smallest value and if only k are stored then it is the kth largest
        self.k_top.peek().unwrap().0
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut kth_largest = KthLargest::new(3, [4, 5, 8, 2].into());
        assert_eq!(kth_largest.add(3), 4); // return 4
        assert_eq!(kth_largest.add(5), 5); // return 5
        assert_eq!(kth_largest.add(10), 5); // return 5
        assert_eq!(kth_largest.add(9), 8); // return 8
        assert_eq!(kth_largest.add(4), 8); // return 8
    }
}
