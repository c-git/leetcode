use std::f32;

impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        // Any two points are on a straight line
        if coordinates.len() <= 2 {
            return true;
        }

        // Extract first two points
        let (x1, y1) = (coordinates[0][0], coordinates[0][1]);
        let (x2, y2) = (coordinates[1][0], coordinates[1][1]);

        // Equation of a line: y = mx + c
        let m = if x1 == x2 {
            // Both x values equal no run (change in x)
            None
        } else {
            // Calculate rise over run (change in y divided by change in x)
            Some((y1 - y2) as f32 / (x1 - x2) as f32)
        };

        // Find line equation and skip first two as they define the line
        if let Some(m) = m {
            // Finish rest of line equation and test rest of points
            let c = y1 as f32 - m * x1 as f32;
            for point in coordinates.iter().skip(2) {
                let (x, y) = (point[0], point[1]);
                let actual_y = y as f32;
                let expected_y = m * x as f32 + c;
                if (actual_y - expected_y).abs() > f32::EPSILON {
                    if cfg!(debug_assertions) {
                        println!(
                            "expected_y = mx + c -> {expected_y} = {m} * {x} + {c}\nactual_y = {actual_y}\nactual_y - expected_y = {} > {}",
                            (actual_y - expected_y).abs(),
                            f32::EPSILON
                        );
                    }
                    return false;
                }
            }
        } else {
            // This is a vertical line all points must have the same x, y does not matter.
            for point in coordinates.iter().skip(2) {
                let x = point[0];
                if x != x1 {
                    return false;
                }
            }
        }

        true
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![[1,2],[2,3],[3,4],[4,5],[5,6],[6,7]], true)]
    #[case(vec![[1,1],[2,2],[3,4],[4,5],[5,6],[7,7]], false)]
    fn case(#[case] input: Vec<[i32; 2]>, #[case] expected: bool) {
        let input = input.into_iter().map(|x| x.into()).collect();
        let actual = Solution::check_straight_line(input);
        assert_eq!(actual, expected);
    }
}
