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
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut result = vec![vec![0; n]; n];

        let (mut bound_top, mut bound_left, mut bound_bottom, mut bound_right) =
            (0, 0, result.len() - 1, result[0].len() - 1);
        let mut pos = Position { row: 0, col: 0 };
        let mut next_value = 1;
        let mut direction = Right;
        while bound_top <= bound_bottom && bound_left <= bound_right {
            if cfg!(debug_assert) {
                dbg!(&pos);
                println!("{result:?}");
            }
            debug_assert!(
                pos.row >= bound_top
                    && pos.row <= bound_bottom
                    && pos.col >= bound_left
                    && pos.col <= bound_right,
                    "{pos:?} Bounds left: {bound_left} right: {bound_right} top: {bound_top} bottom: {bound_bottom}"
            );
            result[pos.row][pos.col] = next_value;
            next_value += 1;

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

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = 3;
        let expected = [
            [1, 2, 3], //
            [8, 9, 4],
            [7, 6, 5],
        ];
        let actual = Solution::generate_matrix(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let input = 1;
        let expected = [[1]];
        let actual = Solution::generate_matrix(input);
        assert_eq!(actual, expected);
    }
}
