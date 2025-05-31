//! Solution for https://leetcode.com/problems/find-median-from-data-stream
//! 295. Find Median from Data Stream

use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Debug, Default)]
struct MedianFinder {
    /// Values smaller than or equal to median
    smaller: BinaryHeap<i32>,

    /// Values larger than or equal to the median
    larger: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        Self::default()
    }

    fn add_num(&mut self, num: i32) {
        self.smaller.push(num);

        // Balance if 'smaller' is more than one larger than 'larger' and if max of
        // smaller is more than min of larger then swap values
        loop {
            if let (Some(max), Some(Reverse(min))) = (self.smaller.peek(), self.larger.peek()) {
                if max > min {
                    // Swap values
                    let max = self.smaller.pop().unwrap();
                    let Reverse(min) = self.larger.pop().unwrap();
                    self.smaller.push(min);
                    self.larger.push(Reverse(max));
                    continue;
                }
            }
            if self.smaller.len() > self.larger.len() + 1 {
                // Move values over until balanced
                let value = self.smaller.pop().unwrap();
                self.larger.push(Reverse(value));
                continue;
            }

            // Balanced Size and well ordered (all smaller are smaller)
            break;
        }
    }

    fn find_median(&self) -> f64 {
        if (self.smaller.len() + self.larger.len()) % 2 == 0 {
            // Even use value from both
            let max = self.smaller.peek().unwrap();
            let Reverse(min) = self.larger.peek().unwrap();
            (min + max) as f64 / 2.0
        } else {
            // Odd use only smaller
            *self.smaller.peek().unwrap() as f64
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
