//! Solution for https://leetcode.com/problems/shortest-path-to-get-all-keys
//! 864. Shortest Path to Get All Keys

impl Solution {
    // Source: sak96 https://github.com/sak96/leet/blob/master/src/shortest_path_to_get_all_keys.rs
    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        const NEIGHBOURS: [[usize; 2]; 4] = [[0, 1], [0, usize::MAX], [1, 0], [usize::MAX, 0]];
        const CHAR_FIRST_KEY: u8 = b'a';
        const CHAR_FIRST_LOCK: u8 = b'A';
        const CHAR_WALL: u8 = b'#';
        const CHAR_START: u8 = b'@';
        const CHAR_EMPTY: u8 = b'.';

        // Convert grid into 2D matrix of bytes
        let grid: Vec<_> = grid.into_iter().map(|s| s.into_bytes()).collect();

        // Collect total number of keys and find start location
        let mut keys_left = 0u32;
        let mut start = None;
        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if cell.is_ascii_lowercase() {
                    keys_left |= 1 << (cell - CHAR_FIRST_KEY) as u32
                }
                if cell.eq(&CHAR_START) {
                    start = Some((row_idx, col_idx));
                }
            }
        }

        debug_assert!(start.is_some());

        let mut seen = std::collections::HashSet::new();
        let mut distance = 0;
        let mut queue = std::collections::VecDeque::new();
        if let Some((row, col)) = start {
            queue.push_front((row, col, keys_left));
            seen.insert((row, col, keys_left));
        } else {
            unreachable!("The should Always be a start point");
        }
        while !queue.is_empty() {
            distance += 1;
            for _ in 0..queue.len() {
                let (row, col, keys_left) = queue.pop_back().unwrap();
                for [r, c] in NEIGHBOURS.iter() {
                    let r = row.wrapping_add(*r);
                    let c = col.wrapping_add(*c);
                    if let Some(&value) = grid.get(r).and_then(|x| x.get(c)) {
                        let keys_left = match value {
                            CHAR_WALL => {
                                // ignore wall
                                continue;
                            }
                            c if c.is_ascii_lowercase() => {
                                // key
                                let keys_left = keys_left & !(1 << (c - CHAR_FIRST_KEY) as u32);
                                if keys_left == 0 {
                                    // No more keys left, we've found the last one
                                    return distance;
                                }
                                keys_left
                            }
                            c if c.is_ascii_uppercase() => {
                                // handle lock
                                if keys_left & 1 << (c - CHAR_FIRST_LOCK) as u32 != 0 {
                                    continue; // not yet unlocked
                                }
                                keys_left
                            }
                            CHAR_EMPTY | CHAR_START => keys_left, // handle traversal
                            _ => unreachable!(
                                "All cases should have been handled but found: {value}"
                            ),
                        };
                        let new_state = (r, c, keys_left);
                        if !seen.contains(&new_state) {
                            queue.push_front(new_state);
                            seen.insert(new_state);
                        }
                    }
                }
            }
        }
        -1 // No path found
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec!["@.a..".into(),"###.#".into(),"b.A.B".into()], 8)]
    #[case(vec!["@..aA".into(),"..B#.".into(),"....b".into()], 6)]
    #[case(vec!["@Aa".into()], -1)]
    #[case(vec!["@abcdeABCDEFf".into()], -1)]
    fn case(#[case] grid: Vec<String>, #[case] expected: i32) {
        let actual = Solution::shortest_path_all_keys(grid);
        assert_eq!(actual, expected);
    }
}
