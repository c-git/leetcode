//! Solution for https://leetcode.com/problems/find-median-from-data-stream
//! 295. Find Median from Data Stream

use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Default)]
struct MedianFinder {
    smaller: BinaryHeap<i32>,
    larger: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        Default::default()
    }

    fn add_num(&mut self, num: i32) {
        self.smaller.push(num);
        while let (Some(smaller), Some(Reverse(larger))) = (self.smaller.peek(), self.larger.peek())
            && smaller > larger
        {
            let old_smaller = self.smaller.pop().expect("condition checks for non-empty");
            let Reverse(old_larger) = self.larger.pop().expect("condition checks for non-empty");
            self.smaller.push(old_larger);
            self.larger.push(Reverse(old_smaller));
        }
        while self.smaller.len() - 1 > self.larger.len() {
            let value = self
                .smaller
                .pop()
                .expect("would have errored with underflow on subtraction if empty already");
            self.larger.push(Reverse(value));
        }
    }

    fn find_median(&self) -> f64 {
        debug_assert!(self.smaller.len() >= self.larger.len());
        if self.smaller.len() == self.larger.len() {
            let smaller = self
                .smaller
                .peek()
                .expect("should never be called if on empty object");
            let larger = self
                .larger
                .peek()
                .expect("should never be called if on empty object");
            (smaller + larger.0) as f64 / 2.0
        } else {
            *self
                .smaller
                .peek()
                .expect("should never be called if on empty object") as f64
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manual() {
        let mut median_finder = MedianFinder::new();
        median_finder.add_num(1); // arr = [1]
        median_finder.add_num(2); // arr = [1, 2]
        assert_eq!(median_finder.find_median(), 1.5); // return 1.5 (i.e., (1 + 2) / 2)
        median_finder.add_num(3); // arr[1, 2, 3]
        assert_eq!(median_finder.find_median(), 2.0); // return 2.0
    }
}
