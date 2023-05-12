use std::cmp::max;

impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let mut dp = vec![0; questions.len()];

        for i in (0..questions.len()).rev() {
            let (points, brain_power) = (questions[i][0] as i64, questions[i][1] as usize);

            let option1_answer_this_question =
                points + dp.get(i + 1 + brain_power).copied().unwrap_or_default();
            let option2_skip_this_question = dp.get(i + 1).copied().unwrap_or_default();
            dp[i] = max(option1_answer_this_question, option2_skip_this_question);
        }
        dp[0]
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = [[3, 2], [4, 3], [4, 4], [2, 5]];
        let input = input.into_iter().map(|x| x.into()).collect();
        let expected = 5;
        let actual = Solution::most_points(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let input = [[1, 1], [2, 2], [3, 3], [4, 4], [5, 5]];
        let input = input.into_iter().map(|x| x.into()).collect();
        let expected = 7;
        let actual = Solution::most_points(input);
        assert_eq!(actual, expected);
    }
}
