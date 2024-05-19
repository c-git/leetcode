//! Solution for https://leetcode.com/problems/edit-distance
//! 72. Edit Distance

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        // Looked at the solutions in particular https://leetcode.com/problems/edit-distance/solutions/3790082/rust-recursion-memoization-top-down-tabulation-bottom-up-space-optimization/
        // which inspired me to use DP myself to solve it

        // General solution idea
        // Use a table `dp` and store the minimum number of edits to make word1 become word 2 for each length of word1 and word2
        // Fill the table from top to bottom
        // For each cell we can get there in three ways given that all cells above and to the left have word1 already equal to word2
        // 1. Use the cell to the left and extend by adding the new character from word2 (+1)
        // 2. Use the cell above and extend by removing the character from word1 (+1)
        // 3. Use the cell diagonally up and left and replace the character from word1 with the char from word2 (+1 if different characters)

        // Convert to arrays to make code simpler for indexing
        let word1 = word1.into_bytes();
        let word2 = word2.into_bytes();

        // dp stores the minimum number of edits required to go from word1 to word2
        // dp is indexed by the number of characters from (word1, word2) (using length of substring)
        let mut dp = vec![vec![0; word2.len() + 1]; word1.len() + 1];

        // Initialize first row where number of characters is 0 from word1
        for (i, cell) in dp[0].iter_mut().enumerate() {
            // Each position we need to do an addition as there are no chars from word1 to be used
            *cell = i; //dp[0][i] = i;
        }

        // Initialize first column of where number of characters from word2 is 0
        for (i, row) in dp.iter_mut().enumerate() {
            // Each position we need to do a deletion as there are no chars from word2 to be used
            row[0] = i; //dp[i][0] = i;
        }

        // See explanation at top for how each value is determined
        for (idx1, c1) in word1.iter().enumerate() {
            for (idx2, c2) in word2.iter().enumerate() {
                // LI:
                //  - dp index is one more than word idx because dp is indexed by length
                //  - All dp with first index <= idx1 are already calculated
                //  - All dp with first index == idx1 +1 and second index <= idx2 are already calculated

                // Add char from word2 so add to cell on the left
                let addition = dp[idx1 + 1][idx2] + 1;

                // Delete char from word1 so add to cell above
                let deletion = dp[idx1][idx2 + 1] + 1;

                // Replace char from word1 with char from word2
                let replace = dp[idx1][idx2] + if c1 == c2 { 0 } else { 1 };

                dp[idx1 + 1][idx2 + 1] = addition.min(deletion).min(replace);
            }
        }

        // The must be at least 1 row and at least 1 column because len+1 is always at least 1
        *dp.last().unwrap().last().unwrap() as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("horse", "ros", 3)]
    #[case("intention", "execution", 5)]
    #[case("", "", 0)]
    #[case("abc", "abc", 0)]
    #[case("abcd", "abc", 1)]
    #[case("aabc", "abc", 1)]
    #[case("abbc", "abc", 1)]
    #[case("abc", "abbc", 1)]
    fn case(#[case] word1: String, #[case] word2: String, #[case] expected: i32) {
        let actual = Solution::min_distance(word1, word2);
        assert_eq!(actual, expected);
    }
}
