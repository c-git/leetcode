use Direction::*;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    #[must_use]
    fn next_direction(self) -> Direction {
        match self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        }
    }
}

#[derive(Debug)]
struct Position {
    row: usize,
    col: usize,
}

impl Solution {
    pub fn spiral_matrix(m: i32, n: i32, mut head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let num_rows = m as usize;
        let num_cols = n as usize;
        let mut result = vec![vec![-1; num_cols]; num_rows];

        let (mut bound_top, mut bound_left, mut bound_bottom, mut bound_right) =
            (0, 0, result.len() - 1, result[0].len() - 1);
        let mut pos = Position { row: 0, col: 0 };
        let mut direction = Right;
        while bound_top <= bound_bottom && bound_left <= bound_right {
            if cfg!(debug_assert) {
                println!("{result:?}");
            }
            debug_assert!(
                pos.row >= bound_top
                    && pos.row <= bound_bottom
                    && pos.col >= bound_left
                    && pos.col <= bound_right,
                    "{pos:?} Bounds left: {bound_left} right: {bound_right} top: {bound_top} bottom: {bound_bottom}"
            );
            result[pos.row][pos.col] = match head {
                Some(node) => {
                    head = node.next;
                    node.val
                }
                None => break,
            };

            // Bounds check
            match (&pos, &direction) {
                (Position { row, col: _ }, Up) if *row == bound_top => {
                    bound_left += 1;
                    direction = direction.next_direction();
                }
                (Position { row, col: _ }, Down) if *row == bound_bottom => {
                    if bound_right == 0 {
                        // Prevent underflow during subtraction
                        break;
                    }
                    bound_right -= 1;
                    direction = direction.next_direction();
                }
                (Position { row: _, col }, Left) if *col == bound_left => {
                    bound_bottom -= 1;
                    direction = direction.next_direction();
                }
                (Position { row: _, col }, Right) if *col == bound_right => {
                    bound_top += 1;
                    direction = direction.next_direction();
                }
                _ => (),
            }

            // Move forward
            match &direction {
                Up => pos.row -= 1,
                Down => pos.row += 1,
                Left => pos.col -= 1,
                Right => pos.col += 1,
            }
        }
        result
    }
}

use crate::helper::ListNode;
struct Solution;
#[cfg(test)]
mod tests {
    use crate::helper::ListHead;

    use super::*;

    #[test]
    fn case1() {
        let m = 3;
        let n = 5;
        let head: ListHead = vec![3, 0, 2, 6, 8, 1, 7, 9, 4, 2, 5, 5, 0].into();
        let expected = [
            [3, 0, 2, 6, 8], //
            [5, 0, -1, -1, 1],
            [5, 2, 4, 9, 7],
        ];
        let actual = Solution::spiral_matrix(m, n, head.into());
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let m = 1;
        let n = 4;
        let head: ListHead = vec![0, 1, 2].into();
        let expected = [[0, 1, 2, -1]];
        let actual = Solution::spiral_matrix(m, n, head.into());
        assert_eq!(actual, expected);
    }
}
