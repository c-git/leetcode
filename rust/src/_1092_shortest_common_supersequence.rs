//! Solution for https://leetcode.com/problems/shortest-common-supersequence
//! 1092. Shortest Common Supersequence

#[derive(Clone, Copy)]
enum FromCell {
    Up,
    Left,
    UpLeft,
}

#[derive(Default, Clone, Copy)]
struct DpInfo {
    from: Option<FromCell>,
    chars_to_add: [Option<char>; 2],
    length: usize,
}

impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        // These are shortest common supersequence for the sub strings
        // Each row represents another letter added from str1
        // Each col represents another letter added from str2
        // Plus one empty row for no values from that string
        let mut dp: Vec<Vec<DpInfo>> =
            vec![vec![Default::default(); str2.len() + 1]; str1.len() + 1];

        // Initialize `dp`
        for (dp_row_idx, c_row) in str1.char_indices() {
            let dp_row_idx = dp_row_idx + 1;
            dp[dp_row_idx][0] = DpInfo {
                from: Some(FromCell::Up),
                chars_to_add: [Some(c_row), None],
                length: dp[dp_row_idx - 1][0].length + 1,
            };
        }
        for (dp_col_idx, c_col) in str2.char_indices() {
            let dp_col_idx = dp_col_idx + 1;
            dp[0][dp_col_idx] = DpInfo {
                from: Some(FromCell::Left),
                chars_to_add: [Some(c_col), None],
                length: dp[0][dp_col_idx - 1].length + 1,
            };
        }
        #[cfg(debug_assertions)]
        print_dp(&dp, &str1, &str2); // Used to visualize failed solutions

        // Fill table
        for (dp_row_idx, c_row) in str1.char_indices() {
            let dp_row_idx = dp_row_idx + 1; // Inc for empty row
            for (dp_col_idx, c_col) in str2.char_indices() {
                let dp_col_idx = dp_col_idx + 1; // Inc for empty col
                let mut candidate = DpInfo {
                    from: None,
                    chars_to_add: [None, None],
                    length: usize::MAX,
                };
                if c_row == c_col {
                    let mut proposed_length = dp[dp_row_idx - 1][dp_col_idx - 1].length + 1;
                    if proposed_length < candidate.length {
                        candidate = DpInfo {
                            from: Some(FromCell::UpLeft),
                            chars_to_add: [Some(c_row), None],
                            length: proposed_length,
                        };
                    }

                    proposed_length = dp[dp_row_idx - 1][dp_col_idx].length + 1;
                    if proposed_length < candidate.length {
                        candidate = DpInfo {
                            from: Some(FromCell::Up),
                            chars_to_add: [Some(c_row), None],
                            length: proposed_length,
                        };
                    }

                    proposed_length = dp[dp_row_idx][dp_col_idx - 1].length + 1;
                    if proposed_length < candidate.length {
                        candidate = DpInfo {
                            from: Some(FromCell::Left),
                            chars_to_add: [Some(c_row), None],
                            length: proposed_length,
                        };
                    }
                } else {
                    let mut proposed_length = dp[dp_row_idx - 1][dp_col_idx - 1].length + 2;
                    if proposed_length < candidate.length {
                        candidate = DpInfo {
                            from: Some(FromCell::UpLeft),
                            chars_to_add: [Some(c_row), Some(c_col)],
                            length: proposed_length,
                        };
                    }

                    proposed_length = dp[dp_row_idx - 1][dp_col_idx].length + 1;
                    if proposed_length < candidate.length {
                        candidate = DpInfo {
                            from: Some(FromCell::Up),
                            chars_to_add: [Some(c_row), None],
                            length: proposed_length,
                        };
                    }

                    proposed_length = dp[dp_row_idx][dp_col_idx - 1].length + 1;
                    if proposed_length < candidate.length {
                        candidate = DpInfo {
                            from: Some(FromCell::Left),
                            chars_to_add: [Some(c_col), None],
                            length: proposed_length,
                        };
                    }
                }
                dp[dp_row_idx][dp_col_idx] = candidate;
                #[cfg(debug_assertions)]
                print_dp(&dp, &str1, &str2); // Used to visualize failed solutions
            }
        }

        // Build final string
        let (mut curr_row, mut curr_col) = (str1.len(), str2.len()); // Index of last cell
        let mut result = vec![];
        loop {
            let curr = &dp[curr_row][curr_col];
            if let Some(c) = curr.chars_to_add[1] {
                result.push(c);
            }
            if let Some(c) = curr.chars_to_add[0] {
                result.push(c);
            }
            if let Some(from_cell) = curr.from.as_ref() {
                match from_cell {
                    FromCell::Up => curr_row -= 1,
                    FromCell::Left => curr_col -= 1,
                    FromCell::UpLeft => {
                        curr_row -= 1;
                        curr_col -= 1;
                    }
                }
            } else {
                // Start of string
                break;
            }
        }

        result.into_iter().rev().collect()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

impl FromCell {
    fn as_str(&self) -> &'static str {
        match self {
            FromCell::Up => "↑",
            FromCell::Left => "←",
            FromCell::UpLeft => "↖",
        }
    }
}
#[cfg(debug_assertions)]
fn print_dp(dp: &[Vec<DpInfo>], str1: &str, str2: &str) {
    println!();
    let col_count = dp[0].len();
    print!("  ");
    for col_idx in 0..col_count {
        print!(
            " |  {col_idx}   {} ",
            if col_idx > 0 {
                str2.chars().nth(col_idx - 1).unwrap()
            } else {
                ' '
            }
        );
    }
    println!(" |");
    print!("---");
    for _ in 0..col_count {
        print!("|---------");
    }
    println!("|");

    for (row_idx, row) in dp.iter().enumerate() {
        print!(
            "{row_idx} {}",
            if row_idx > 0 {
                str1.chars().nth(row_idx - 1).unwrap()
            } else {
                ' '
            }
        );
        for cell in row {
            print!(
                "| {} {}{} ({})",
                cell.from.map(|x| x.as_str()).unwrap_or("-"),
                cell.chars_to_add[0].unwrap_or(' '),
                cell.chars_to_add[1].unwrap_or(' '),
                cell.length
            );
        }
        println!("|");
        print!("---");
        for _ in 0..col_count {
            print!("|---------");
        }
        println!("|");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("abac", "cab", "cabac")]
    #[case("aaaaaaaa", "aaaaaaaa", "aaaaaaaa")]
    fn case(#[case] str1: String, #[case] str2: String, #[case] expected: String) {
        let actual = Solution::shortest_common_supersequence(str1, str2);
        assert_eq!(actual, expected);
    }
}
