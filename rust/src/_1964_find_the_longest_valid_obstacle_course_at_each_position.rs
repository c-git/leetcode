use std::collections::BTreeMap;

impl Solution {
    pub fn longest_obstacle_course_at_each_position(obstacles: Vec<i32>) -> Vec<i32> {
        let mut largest_seen: BTreeMap<i32, i32> = BTreeMap::new();
        let mut result: Vec<i32> = Vec::with_capacity(obstacles.len());
        for current_height in obstacles {
            let mut max_prev_len = 0;
            for (&prev_height, &prev_length) in largest_seen.iter() {
                if prev_height > current_height {
                    break; // No more useful values to look at
                }
                if prev_length > max_prev_len {
                    max_prev_len = prev_length;
                }
            }
            let new_length = max_prev_len + 1;
            result.push(new_length);

            // This new height must be higher than any with the same height because
            // it would be at least 1 longer, because it would have extended the previous
            // longest of the same height if that was the longest previous
            largest_seen.insert(current_height, new_length);
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
        let expected: Vec<i32> = (1..=size).map(|x| x as i32).collect();
        let actual = Solution::longest_obstacle_course_at_each_position(input);
        assert!(actual == expected);
    }

    #[test]
    fn case6() {
        // Got a timeout but the test case was too big. Just adding this to help gauge the time better
        let size = 10_000;
        let input: Vec<i32> = (1..=size).collect();
        let expected: Vec<i32> = input.clone();
        let actual = Solution::longest_obstacle_course_at_each_position(input);
        assert!(actual == expected);
    }
}
