impl Solution {
    pub fn longest_obstacle_course_at_each_position(obstacles: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::with_capacity(obstacles.len());
        for (outer_index, outer_height) in obstacles.iter().enumerate() {
            let mut max_prev_len = 0;
            for (inner_index, inner_height) in obstacles.iter().take(outer_index).enumerate().rev()
            {
                if inner_height <= outer_height && max_prev_len < result[inner_index] {
                    max_prev_len = result[inner_index]
                }
            }
            result.push(max_prev_len + 1);
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
        let input = vec![1, 2, 3, 2];
        let expected = vec![1, 2, 3, 3];
        let actual = Solution::longest_obstacle_course_at_each_position(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let input = vec![2, 2, 1];
        let expected = vec![1, 2, 1];
        let actual = Solution::longest_obstacle_course_at_each_position(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case3() {
        let input = vec![3, 1, 5, 6, 4, 2];
        let expected = vec![1, 1, 2, 3, 2, 2];
        let actual = Solution::longest_obstacle_course_at_each_position(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case4() {
        let input = vec![5, 1, 5, 5, 1, 3, 4, 5, 1, 4];
        let expected = vec![1, 1, 2, 3, 2, 3, 4, 5, 3, 5];
        let actual = Solution::longest_obstacle_course_at_each_position(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case5() {
        // Got a timeout but the test case was too big. Just adding this to help gauge the time better
        let size = 10_000;
        let input = vec![1; size];
        let expected: Vec<i32> = (1..size).map(|x| x as i32).collect();
        let actual = Solution::longest_obstacle_course_at_each_position(input);
        assert_eq!(actual, expected);
    }
}
