impl Solution {
    // Source: sak96
    pub fn stone_game_ii_helper(
        piles: &[i32],
        sum_piles: i32,
        m: usize,
        memo: &mut Vec<Vec<Option<i32>>>,
    ) -> i32 {
        debug_assert_eq!(sum_piles, piles.iter().sum::<i32>());
        let key = (piles.len(), m);
        let value = if piles.len() <= 2 * m {
            // take all you can and give nothing back
            sum_piles
        } else if let Some(value) = memo[key.0][key.1] {
            return value;
        } else {
            // pick move that maximizes final score
            // final score = current piles sum - opponents best move
            let mut max_score = 0;
            let mut sum_remaining = sum_piles;
            for i in 1..=2 * m {
                sum_remaining -= piles[i - 1];
                let score = sum_piles
                    - Self::stone_game_ii_helper(&piles[i..], sum_remaining, i.max(m), memo);
                max_score = max_score.max(score);
            }
            max_score
        };
        memo[key.0][key.1] = Some(value);
        value
    }

    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let sum: i32 = piles.iter().sum();
        let n = piles.len();
        let mut memo = vec![vec![None; n + 1]; n + 1];
        Self::stone_game_ii_helper(&piles, sum, 1, &mut memo)
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![2,7,9,4,4],10)]
    #[case(vec![1,2,3,4,5,100],104)]
    #[case(vec![1,1,1,1,1,1,1,1,1,1],6)]
    fn case(#[case] input: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::stone_game_ii(input);
        assert_eq!(actual, expected);
    }
}
