impl Solution {
    pub fn maximum_detonation(mut bombs: Vec<Vec<i32>>) -> i32 {
        // General idea is that if we sort the bombs in ascending order of their blast radius then we can solve
        // the problem for prefixes of the list and know that we are not missing anything because anything it can reach
        // that we haven't checked yet can also reach it so we'll just blow up that one instead
        // and once we extend the maximum of the previous list then we must get the overall maximum

        let mut dp = vec![1; bombs.len()];
        bombs.sort_unstable_by_key(|x| x[2]);
        for (this_ind, this_bomb) in bombs.iter().enumerate().skip(1) {
            let (this_x, this_y, r) = (this_bomb[0], this_bomb[1], this_bomb[2]);
            let blast_radius = r as f32;
            for (other_ind, other_bomb) in bombs.iter().enumerate().take(this_ind).rev() {
                let (other_x, other_y) = (other_bomb[0], other_bomb[1]);
                let distance =
                    (((this_x - other_x).pow(2) + (this_y - other_y).pow(2)) as f32).sqrt();
                if distance <= blast_radius {
                    let candidate_count = 1 + dp[other_ind];
                    if dp[this_ind] < candidate_count {
                        dp[this_ind] = candidate_count;
                    }
                }
            }
        }

        *dp.iter().max().unwrap()
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![[2,1,3],[6,1,4]], 2)]
    #[case(vec![[1,1,5],[10,10,5]], 1)]
    #[case(vec![[1,2,3],[2,3,1],[3,4,2],[4,5,3],[5,6,4]], 5)]
    fn case(#[case] input: Vec<[i32; 3]>, #[case] expected: i32) {
        let input = input.into_iter().map(|x| x.into()).collect();
        let actual = Solution::maximum_detonation(input);
        assert_eq!(actual, expected);
    }
}
