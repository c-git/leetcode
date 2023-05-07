impl Solution {
    pub fn longest_obstacle_course_at_each_position(obstacles: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::with_capacity(obstacles.len());
        for (i, curr_height) in obstacles.iter().enumerate() {
            let len = 1 + obstacles
                .iter()
                .take(i)
                .enumerate()
                .rev()
                .find_map(|(obstacle_index, obstacle_height)| {
                    if obstacle_height <= curr_height {
                        Some(result[obstacle_index])
                    } else {
                        None
                    }
                })
                .unwrap_or_default();
            result.push(len);
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
}
