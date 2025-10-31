//! Solution for https://leetcode.com/problems/find-median-from-data-stream
//! 295. Find Median from Data Stream

use std::{cmp::Reverse, collections::BinaryHeap};

pub struct MedianFinder {
    smaller: BinaryHeap<i32>,
    larger: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    #[expect(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            smaller: Default::default(),
            larger: Default::default(),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        self.smaller.push(num);
        if !self.larger.is_empty() {
            while self.smaller.peek() > self.larger.peek().map(|x| &x.0) {
                self.larger.push(Reverse(self.smaller.pop().unwrap()));
            }
        }
        while self.larger.len() > self.smaller.len() || self.smaller.len() - self.larger.len() > 1 {
            debug_assert_ne!(self.smaller.len(), self.larger.len());
            if self.smaller.len() < self.larger.len() {
                self.smaller.push(self.larger.pop().map(|x| x.0).unwrap());
            } else {
                self.larger.push(Reverse(self.smaller.pop().unwrap()));
            }
        }
    }

    pub fn find_median(&self) -> f64 {
        if self.smaller.len() > self.larger.len() {
            *self.smaller.peek().unwrap() as f64
        } else {
            (*self.smaller.peek().unwrap() as f64 + self.larger.peek().map(|x| x.0).unwrap() as f64)
                / 2.0
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
