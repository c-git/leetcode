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
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() {
            return vec![];
        }

        let mut result = Vec::with_capacity(matrix.len() * matrix[0].len());

        let (mut bound_top, mut bound_left, mut bound_bottom, mut bound_right) =
            (0, 0, matrix.len() - 1, matrix[0].len() - 1);
        let mut pos = Position { row: 0, col: 0 };
        let mut direction = Right;
        while bound_top <= bound_bottom && bound_left <= bound_right {
            dbg!(&pos);
            println!("{result:?}");
            debug_assert!(
                pos.row >= bound_top
                    && pos.row <= bound_bottom
                    && pos.col >= bound_left
                    && pos.col <= bound_right,
                    "{pos:?} Bounds left: {bound_left} right: {bound_right} top: {bound_top} bottom: {bound_bottom}"
            );
            result.push(matrix[pos.row][pos.col]);

            // Bounds check
            match (&pos, &direction) {
                (Position { row, col: _ }, Up) if *row == bound_top => {
                    bound_left += 1;
                    direction = direction.next_direction();
                }
                (Position { row, col: _ }, Down) if *row == bound_bottom => {
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
        let input = [
            [1, 2, 3], // To put multiline
            [4, 5, 6],
            [7, 8, 9],
        ];
        let input = input.into_iter().map(|x| x.into()).collect();
        let expected = [1, 2, 3, 6, 9, 8, 7, 4, 5];
        let actual = Solution::spiral_order(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let input = [
            [1, 2, 3, 4], //
            [5, 6, 7, 8],
            [9, 10, 11, 12],
        ];
        let input = input.into_iter().map(|x| x.into()).collect();
        let expected = [1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7];
        let actual = Solution::spiral_order(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case3() {
        let input = [
            [11, 12, 13, 14, 15],
            [21, 22, 23, 24, 25],
            [31, 32, 33, 34, 35],
            [41, 42, 43, 44, 45],
            [51, 52, 53, 54, 55],
            [61, 62, 63, 64, 65],
            [71, 72, 73, 74, 75],
            [81, 82, 83, 84, 85],
        ];
        let input = input.into_iter().map(|x| x.into()).collect();
        let expected = [
            11, 12, 13, 14, 15, 25, 35, 45, 55, 65, 75, 85, 84, 83, 82, 81, 71, 61, 51, 41, 31, 21,
            22, 23, 24, 34, 44, 54, 64, 74, 73, 72, 62, 52, 42, 32, 33, 43, 53, 63,
        ];
        let actual = Solution::spiral_order(input);
        assert_eq!(actual, expected);
    }
}
