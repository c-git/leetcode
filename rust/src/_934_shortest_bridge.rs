use CellType::*;
use Direction::*;

#[derive(Debug, PartialEq, Eq)]
enum CellType {
    Water,
    Land,
    Visited,
}

impl From<&i32> for CellType {
    fn from(value: &i32) -> Self {
        match value {
            0 => Water,
            1 => Land,
            2 => Visited,
            _ => unreachable!(),
        }
    }
}

impl From<CellType> for i32 {
    fn from(value: CellType) -> Self {
        match value {
            Water => 0,
            Land => 1,
            Visited => 2,
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<Direction> for (i32, i32) {
    fn from(value: Direction) -> Self {
        match value {
            Up => (-1, 0),
            Down => (1, 0),
            Left => (0, -1),
            Right => (0, 1),
        }
    }
}

impl Direction {
    fn all() -> [Direction; 4] {
        [Up, Down, Left, Right]
    }

    fn neighbours(pos: (i32, i32), n: i32) -> Vec<(i32, i32)> {
        let mut result = Vec::with_capacity(4);
        for direction in Self::all() {
            let dir: (i32, i32) = direction.into();
            let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if new_pos.0 >= 0 && new_pos.0 < n && new_pos.1 >= 0 && new_pos.1 < n {
                result.push(new_pos);
            }
        }
        result
    }
}

impl Solution {
    pub fn shortest_bridge(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len() as i32;

        if cfg!(debug_assertions) {
            for row in grid.iter() {
                println!("{row:?}")
            }
            println!();
        }

        // Find top left edge of first island
        let mut top_left_edge = (0, 0);
        'outer: for (row_index, row) in grid.iter().enumerate() {
            for (col_index, cell_value) in row.iter().enumerate() {
                let cell_type: CellType = cell_value.into();
                if cell_type == Land {
                    top_left_edge = (row_index as i32, col_index as i32);
                    break 'outer;
                }
            }
        }

        // Mark first island all as visited and track all edges with water
        grid[top_left_edge.0 as usize][top_left_edge.1 as usize] = Visited.into();
        let mut stack = vec![top_left_edge];
        while let Some(pos) = stack.pop() {
            for neighbour in Direction::neighbours(pos, n) {
                if grid[neighbour.0 as usize][neighbour.1 as usize] == Land.into() {
                    grid[neighbour.0 as usize][neighbour.1 as usize] = Visited.into();
                    stack.push(neighbour);
                }
            }
        }

        if cfg!(debug_assertions) {
            for row in grid.iter() {
                println!("{row:?}")
            }
        }

        todo!()
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = [
            [0, 1], //
            [1, 0],
        ];
        let input = input.into_iter().map(|x| x.into()).collect();
        let expected = 1;
        let actual = Solution::shortest_bridge(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let input = [
            [0, 1, 0], //
            [0, 0, 0],
            [0, 0, 1],
        ];
        let input = input.into_iter().map(|x| x.into()).collect();
        let expected = 2;
        let actual = Solution::shortest_bridge(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case3() {
        let input = [
            [1, 1, 1, 1, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 1, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 1, 1, 1, 1],
        ];
        let input = input.into_iter().map(|x| x.into()).collect();
        let expected = 1;
        let actual = Solution::shortest_bridge(input);
        assert_eq!(actual, expected);
    }
}
