//! Solution for https://leetcode.com/problems/longest-common-subsequence
//! 1143. Longest Common Subsequence

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        // Taken from https://leetcode.com/problems/longest-common-subsequence/solutions/2394254/rust-dp-with-comments/

        // Sort string by longer and shorter to reduce the memory usage
        let (shorter, longer) = if text1.len() < text2.len() {
            (text1, text2)
        } else {
            (text2, text1)
        };

        // Input is ASCII => chars are bytes
        let (longer, shorter) = (longer.as_bytes(), shorter.as_bytes());

        // Initialize previous DP row. All zeros represent taking no characters from text1
        let mut dp_prev = vec![0; shorter.len()];
        let mut dp_curr = dp_prev.clone();

        // Iterate in over the text strings, keeping track of the LCS considering the
        // corresponding prefixes
        #[allow(clippy::needless_range_loop)]
        for idx_longer in 0..longer.len() {
            for idx_shorter in 0..shorter.len() {
                // Take the best path - either skipping the current character in shorter, or
                // skipping the current character in longer, or using the characters if they match.
                let (last_curr_value, last_prev_value) = if idx_shorter == 0 {
                    (0, 0)
                } else {
                    (dp_curr[idx_shorter - 1], dp_prev[idx_shorter - 1])
                };
                dp_curr[idx_shorter] = dp_prev[idx_shorter].max(last_curr_value).max(
                    last_prev_value
                        + if longer[idx_longer] == shorter[idx_shorter] {
                            1
                        } else {
                            0
                        },
                );
                #[cfg(debug_assertions)]
                print_debug_info(
                    idx_shorter,
                    idx_longer,
                    &dp_prev,
                    &dp_curr,
                    &shorter,
                    &longer,
                );
            }

            // Swap the rows to reuse dp_prev, which is now stale
            #[cfg(debug_assertions)]
            println!("Swapping arrays now................................\n");
            std::mem::swap(&mut dp_prev, &mut dp_curr);
        }

        *dp_prev.last().unwrap()
    }
}

#[cfg(debug_assertions)]
fn print_debug_info(
    idx_shorter: usize,
    idx_longer: usize,
    dp_prev: &[i32],
    dp_curr: &[i32],
    shorter: &&[u8],
    longer: &&[u8],
) {
    println!(
        "({idx_longer}, {idx_shorter}) - ({}, {})",
        char::from_u32(longer[idx_longer] as _).unwrap(),
        char::from_u32(shorter[idx_shorter] as _).unwrap()
    );
    println!("prev: {dp_prev:?}");
    println!("curr: {dp_curr:?}");
    println!();
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    // #[case("abcde", "ace", 3)]
    // #[case("abcde", "cce", 2)]
    // #[case("abcde", "ce", 2)]
    // #[case("abc", "abc", 3)]
    // #[case("abc", "def", 0)]
    // #[case("abcba", "abcbcba", 5)]
    // #[case("pmjghexybyrgzczy", "hafcdqbgncrcbihkd", 4)]
    #[case("jghbrgc", "hcbgcrcbhk", 4)]
    fn case(#[case] text1: String, #[case] text2: String, #[case] expected: i32) {
        let actual = Solution::longest_common_subsequence(text1, text2);
        assert_eq!(actual, expected);
    }
}
