impl Solution {
    fn most_points_helper(
        starting_index: usize,
        questions: &[Vec<i32>],
        memo: &mut [Option<i64>],
    ) -> i64 {
        if starting_index >= questions.len() {
            return 0;
        }
        if let Some(result) = memo[starting_index] {
            return result;
        }

        let (points, brain_power) = (
            questions[starting_index][0] as i64,
            questions[starting_index][1] as usize,
        );

        let option1_answer_this_question =
            points + Self::most_points_helper(starting_index + 1 + brain_power, questions, memo);
        let option2_skip_this_question =
            Self::most_points_helper(starting_index + 1, questions, memo);
        let result = option1_answer_this_question.max(option2_skip_this_question);
        memo[starting_index] = Some(result);
        result
    }

    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let mut memo = vec![None; questions.len()];
        Self::most_points_helper(0, &questions[..], &mut memo)
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
