//! Solution for https://leetcode.com/problems/k-th-symbol-in-grammar
//! 779. K-th Symbol in Grammar

impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        // After reading editorial
        debug_assert!(k < 2i32.pow(n as u32));
        if n == 1 {
            return 0;
        }
        let count_nodes_in_upper_layer = 2i32.pow(n as u32 - 2);
        let (new_k, is_left_child) = if k <= count_nodes_in_upper_layer {
            // In left subtree
            (k, true)
        } else {
            // In right sub tree discard left side
            (k - count_nodes_in_upper_layer, false)
        };
        let parent = Self::kth_grammar(n - 1, new_k);
        if is_left_child {
            parent
        } else {
            1 - parent
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
    #[case(1, 1, 0)]
    #[case(2, 1, 0)]
    #[case(2, 2, 1)]
    #[case(3, 1, 0)]
    #[case(3, 2, 1)]
    #[case(3, 3, 1)]
    #[case(3, 4, 0)]
    #[case(4, 5, 1)]
    #[case(4, 6, 0)]
    #[case(4, 7, 0)]
    #[case(4, 8, 1)]
    #[case(5, 9, 1)]
    #[case(30, 536870912, 1)]
    fn case(#[case] n: i32, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::kth_grammar(n, k);
        assert_eq!(actual, expected, "n={n}, k={k}");
    }
}
