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
        if cfg!(debug_assertions) {
            println!("Converted and sorted ranges: {ranges:?}");
        }

        // Find minimal required ranges
        let mut current = *ranges.first()?; // If ranges is empty then no solution possible
        result = 1;
        if current[0] > 0 {
            // No ranges cover 0
            return None;
        }
        let mut previously_covered = current[1];
        if previously_covered >= n {
            return Some(result);
        } else {
            result += 1; // Make current be the previous range
        }
        for range in ranges {
            if current[1] >= n {
                // All ranges already covered
                return Some(result);
            }
            let is_equal_start_bound = range[0] == current[0];
            let does_cover_further = range[1] > current[1];
            let is_passed_covered_by_previous = previously_covered < range[0];
            match (
                is_equal_start_bound,
                does_cover_further,
                is_passed_covered_by_previous,
            ) {
                (true, true, _) => unreachable!("Unable to be reached because of sort order"),
                (true, false, _) => {
                    // Less covered just skip this one
                }
                (false, true, true) => {
                    // Covers more than current and starts after the previous so we need to keep the current to cover that partial range
                    if range[0] > current[1] + 1 {
                        // There is now a gap that cannot be covered
                        return None;
                    }
                    result += 1;
                    debug_assert!(
                        previously_covered < current[1],
                        "Must be grater otherwise wouldn't have become current"
                    );
                    previously_covered = current[1];
                    current = range;
                }
                (false, true, false) => {
                    // Covers more than current and the part covered by current right now is still covered by the previous
                    current = range;
                }
                (false, false, _) => {
                    // This range is useless, it is already covered
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
    #[case(8, vec![4,0,0,0,4,0,0,0,4], 1)]
    #[case(9, vec![0,5,0,3,3,3,1,4,0,4], 2)]
    #[case(9, vec![3,0,0,0,0,0,0,0,0,4], -1)]
    fn case(#[case] n: i32, #[case] ranges: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::min_taps(n, ranges);
        assert_eq!(actual, expected);
    }
}
