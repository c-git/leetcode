//! Solution for https://leetcode.com/problems/minimum-recolors-to-get-k-consecutive-black-blocks
//! 2379. Minimum Recolors to Get K Consecutive Black Blocks

impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let k = k as usize;
        let blocks = blocks.as_bytes();
        debug_assert!(k <= blocks.len());
        let mut curr_count = blocks.iter().take(k).filter(|&&x| x == b'B').count();
        if curr_count == k {
            return 0;
        }
        let mut best_count = curr_count;
        for i in k..blocks.len() {
            dbg!(std::str::from_utf8(&[blocks[i]]).unwrap());
            if blocks[i] == b'B' {
                curr_count += 1;
            }
            if blocks[i - k] == b'B' {
                curr_count -= 1;
            }
            if curr_count >= k {
                return 0;
            }
            best_count = best_count.max(curr_count);
        }
        (k - best_count) as _
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("WBBWWBBWBW", 7, 3)]
    #[case("WBWBBBW", 2, 0)]
    #[case("WB", 1, 0)]
    #[case("BWB", 1, 0)]
    fn case(#[case] blocks: String, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::minimum_recolors(blocks, k);
        assert_eq!(actual, expected);
    }
}
