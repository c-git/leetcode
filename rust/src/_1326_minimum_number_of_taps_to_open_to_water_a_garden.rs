//! Solution for https://leetcode.com/problems/minimum-number-of-taps-to-open-to-water-a-garden
//! 1326. Minimum Number of Taps to Open to Water a Garden

impl Solution {
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        Self::min_taps_(n, ranges).unwrap_or(-1)
    }

    fn min_taps_(n: i32, ranges: Vec<i32>) -> Option<i32> {
        let n = n as usize;
        let mut result;
        debug_assert_eq!(ranges.len(), n + 1);

        // Convert ranges into inclusive start stop pairs
        let mut ranges: Vec<[usize; 2]> = ranges
            .into_iter()
            .enumerate()
            .filter_map(|(i, x)| {
                if x > 0 {
                    Some([i.saturating_sub(x as usize), i + x as usize])
                } else {
                    None
                }
            })
            .collect();

        // Sort ranges so that we can solve in only a forward pass
        // All values that can cover a point are grouped together and the one that covers the most comes first
        ranges.sort_unstable_by_key(|x| (x[0], -(x[1] as i32)));

        // Find minimal required ranges
        let mut current = *ranges.first()?; // If ranges is empty then no solution possible
        result = 1;
        for range in ranges {
            match (range[0] == current[0], range[1] > current[1]) {
                (true, true) => unreachable!(
                    "This should never happen because we sorted by reversing ending boundary"
                ),
                (_, false) => {
                    // Ignore this range it doesn't cover as much as the current
                    // This range starts at least as early but goes further
                }
                (false, true) => {
                    // We need this new range because it covers more of the array we cannot cover yet
                    // We need the previous range because this new one doesn't cover the start of the last one
                    result += 1;
                    current = range;
                }
            }
        }

        if current[1] >= n {
            Some(result)
        } else {
            None
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
    #[case(5, vec![3,4,1,1,0,0], 1)]
    #[case(3, vec![0,0,0,0], -1)]
    fn case(#[case] n: i32, #[case] ranges: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::min_taps(n, ranges);
        assert_eq!(actual, expected);
    }
}
