use std::collections::VecDeque;

struct CellData {
    /// Distance from the start
    distance: i32,
    row: i32,
    col: i32,
}

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() || grid[0][0] == 1 {
            return -1;
        }

        let n = grid.len();
        let target = n - 1; // Target cell position index
        let mut is_visited = vec![vec![false; n]; n];
        let n = n as i32; // Converted here to reduce the overall number of conversions required
        let mut queue = VecDeque::new();
        queue.push_back(CellData {
            distance: 1,
            row: 0,
            col: 0,
        });
        is_visited[0][0] = true;

        while let Some(CellData { distance, row, col }) = queue.pop_front() {
            let new_distance = distance + 1;
            for row_offset in -1..=1 {
                let new_row = row + row_offset;
                if (0..n).contains(&new_row) {
                    for col_offset in -1..=1 {
                        let new_col = col + col_offset;
                        if (0..n).contains(&new_col) {
                            let new_row = new_row as usize;
                            let new_col = new_col as usize;
                            if !is_visited[new_row][new_col] && grid[new_row][new_col] == 0 {
                                if new_row == target && new_col == target {
                                    return new_distance;
                                }
                                is_visited[new_row][new_col] = true;
                                queue.push_back(CellData {
                                    distance: new_distance,
                                    row: new_row as i32,
                                    col: new_col as i32,
                                })
                            }
                        }
                    }
                }
            }
        }
        -1 // Unable to reach target cell
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![0,1],vec![1,0]],2)]
    #[case(vec![vec![0,0,0],vec![1,1,0],vec![1,1,0]],4)]
    #[case(vec![vec![1,0,0],vec![1,1,0],vec![1,1,0]],-1)]
    #[case(vec![vec![0,0,0],vec![1,1,0],vec![1,1,1]],-1)]
    fn case(#[case] input: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::shortest_path_binary_matrix(input);
        assert_eq!(actual, expected);
    }
}
