//! Solution for https://leetcode.com/problems/check-if-grid-can-be-cut-into-sections
//! 3394. Check if Grid can be Cut into Sections

impl Solution {
    pub fn check_valid_cuts(_n: i32, mut rectangles: Vec<Vec<i32>>) -> bool {
        if Self::check_valid_cuts_(&mut rectangles, 0) {
            // Horizontal cuts found
            return true;
        }

        Self::check_valid_cuts_(&mut rectangles, 1)
    }

    /// Check if two cuts are possible using the offset provided, 0 =
    /// Horizontal, 1 = Vertical
    fn check_valid_cuts_(rectangles: &mut [Vec<i32>], offset: usize) -> bool {
        let mut cuts_found = 0;

        // Sort by start of rectangles
        rectangles.sort_unstable_by_key(|rectangle| rectangle[offset]);

        let mut end = rectangles[0][2 + offset];
        for rectangle in rectangles.iter() {
            let start = rectangle[offset];
            if start >= end {
                cuts_found += 1;
                if cuts_found >= 2 {
                    return true;
                }
            }
            end = end.max(rectangle[2 + offset]);
        }

        false
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(5, vec![vec![1,0,5,2],vec![0,2,2,4],vec![3,2,5,3],vec![0,4,4,5]], true)]
    #[case(4, vec![vec![0,0,1,1],vec![2,0,3,4],vec![0,2,2,3],vec![3,0,4,3]], true)]
    #[case(4, vec![vec![0,2,2,4],vec![1,0,3,2],vec![2,2,3,4],vec![3,0,4,2],vec![3,2,4,4]], false)]
    fn case(#[case] n: i32, #[case] rectangles: Vec<Vec<i32>>, #[case] expected: bool) {
        let actual = Solution::check_valid_cuts(n, rectangles);
        assert_eq!(actual, expected);
    }
}
