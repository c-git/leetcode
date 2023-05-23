use std::collections::BinaryHeap;

use CellType::*;
use Direction::*;

type Position = (i32, i32); //Uses i32 to be able to subtract below 0 to simplify code

#[derive(Debug, PartialEq, Eq)]
enum CellType {
    Water,
    Land,
    Visited,
}

impl From<i32> for CellType {
    fn from(value: i32) -> Self {
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

impl From<Direction> for Position {
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

    fn neighbours(pos: Position, n: i32) -> Vec<Position> {
        let mut result = Vec::with_capacity(4);
        for direction in Self::all() {
            let dir: Position = direction.into();
            let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if new_pos.0 >= 0 && new_pos.0 < n && new_pos.1 >= 0 && new_pos.1 < n {
                result.push(new_pos);
            }
        }
        result
    }
}

struct CellReference {
    pos: Position,
    distance: i32,
}

impl CellReference {
    fn new(pos: Position) -> Self {
        Self { pos, distance: 1 }
    }

    fn new_with_distance(pos: Position, distance: i32) -> CellReference {
        Self { pos, distance }
    }
}

impl PartialEq for CellReference {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}
impl Eq for CellReference {}

impl PartialOrd for CellReference {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}

impl Ord for CellReference {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl Solution {
    // After reading editorial (There was no better way than what I could think of...)
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
                let cell_type: CellType = (*cell_value).into();
                if cell_type == Land {
                    top_left_edge = (row_index as i32, col_index as i32);
                    break 'outer;
                }
            }
        }

        // Mark first island all as visited and add all adjacent water to the BFS queue
        let mut bfs_queue = BinaryHeap::new();
        grid[top_left_edge.0 as usize][top_left_edge.1 as usize] = Visited.into();
        let mut stack = vec![top_left_edge];
        while let Some(pos) = stack.pop() {
            for neighbour in Direction::neighbours(pos, n) {
                let neighbour_value = &mut grid[neighbour.0 as usize][neighbour.1 as usize];
                let neighbour_type: CellType = (*neighbour_value).into();
                match neighbour_type {
                    Land => {
                        *neighbour_value = Visited.into();
                        stack.push(neighbour); // Record this cell to check it's neighbours for more land
                    }
                    Water => {
                        *neighbour_value = Visited.into();
                        bfs_queue.push(CellReference::new(neighbour)); // Record water found for BFS later
                    }
                    _ => {}
                }
            }
        }

        if cfg!(debug_assertions) {
            for row in grid.iter() {
                println!("{row:?}")
            }
            println!()
        }

        // Walk out from edges of first island until we find the second island (and well the answer)
        while let Some(CellReference { pos, distance }) = bfs_queue.pop() {
            if grid[pos.0 as usize][pos.1 as usize] == Land.into() {
                return distance;
            }
            for neighbour in Direction::neighbours(pos, n) {
                let neighbour_value = &mut grid[neighbour.0 as usize][neighbour.1 as usize];
                let neighbour_type: CellType = (*neighbour_value).into();
                match neighbour_type {
                    Land => {
                        return distance;
                    }
                    Water => {
                        *neighbour_value = Visited.into();
                        bfs_queue.push(CellReference::new_with_distance(neighbour, distance + 1));
                    }
                    _ => {}
                }
            }
        }
        if cfg!(debug_assertions) {
            for row in grid.iter() {
                println!("{row:?}")
            }
        }

        unreachable!("Constraint that second island exists")
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
