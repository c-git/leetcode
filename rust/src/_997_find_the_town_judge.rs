//! Solution for https://leetcode.com/problems/find-the-town-judge
//! 997. Find the Town Judge

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        // Keep track of who is still a candidate for judge
        let mut candidates = vec![true; n as usize + 1];

        // Track how many people trust each person
        let mut number_trusting = vec![0; n as usize + 1];

        for pair in trust {
            let (truster, trustee) = (pair[0] as usize, pair[1] as usize);
            candidates[truster] = false;
            number_trusting[trustee] += 1;
        }

        candidates
            .iter()
            .enumerate()
            .skip(1)
            .find_map(|(i, &can_be_judge)| {
                if can_be_judge {
                    if number_trusting[i] == n - 1 {
                        Some(i as i32)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .unwrap_or(-1)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(2, vec![vec![1,2]], 2)]
    #[case(3, vec![vec![1,3],vec![2,3]], 3)]
    #[case(3, vec![vec![1,3],vec![2,3],vec![3,1]], -1)]
    fn case(#[case] n: i32, #[case] trust: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::find_judge(n, trust);
        assert_eq!(actual, expected);
    }
}
