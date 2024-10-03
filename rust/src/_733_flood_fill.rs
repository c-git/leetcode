//! Solution for https://leetcode.com/problems/flood-fill
//! 733. Flood Fill

impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let pos = [sr as usize, sc as usize];
        let org_color = image[pos[0]][pos[1]];
        if org_color == color {
            // To prevent infinite loop
            return image;
        }
        let mut stack = vec![pos];
        while let Some(pos) = stack.pop() {
            if image[pos[0]][pos[1]] == org_color {
                // Change color
                image[pos[0]][pos[1]] = color;

                // Up
                if pos[0] > 0 {
                    stack.push([pos[0] - 1, pos[1]]);
                }

                // Down
                if pos[0] < image.len() - 1 {
                    stack.push([pos[0] + 1, pos[1]]);
                }

                // Left
                if pos[1] > 0 {
                    stack.push([pos[0], pos[1] - 1]);
                }

                // Right
                if pos[1] < image[0].len() - 1 {
                    stack.push([pos[0], pos[1] + 1]);
                }
            }
        }
        image
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,1,1],vec![1,1,0],vec![1,0,1]], 1, 1, 2, [[2,2,2].into(),[2,2,0].into(),[2,0,1].into()].into())]
    #[case(vec![vec![0,0,0],vec![0,0,0]], 0, 0, 0, [[0,0,0].into(),[0,0,0].into()].into())]
    fn case(
        #[case] image: Vec<Vec<i32>>,
        #[case] sr: i32,
        #[case] sc: i32,
        #[case] color: i32,
        #[case] expected: Vec<Vec<i32>>,
    ) {
        let actual = Solution::flood_fill(image, sr, sc, color);
        assert_eq!(actual, expected);
    }
}
