impl Solution {
    pub fn longest_obstacle_course_at_each_position(obstacles: Vec<i32>) -> Vec<i32> {
        // Solution from editorial
        let n = obstacles.len();
        let mut result = Vec::<i32>::with_capacity(n);

        // lis[i] records the lowest increasing sequence of length i + 1.
        let mut lis = vec![];

        for (i, &height) in obstacles.iter().enumerate() {
            // Find the rightmost insertion position idx.
            let idx = lis.partition_point(|&x| x <= height);

            if idx == lis.len() {
                lis.push(height);
            } else {
                lis[idx] = height;
            }
            result.push((idx + 1) as i32);
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
