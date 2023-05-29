use std::collections::HashMap;

impl Solution {
    pub fn min_cost(n: i32, mut cuts: Vec<i32>) -> i32 {
        // After reading editorial
        let mut memo = HashMap::new();

        cuts.push(0);
        cuts.push(n);
        cuts.sort_unstable();

        Self::cost(0, cuts.len() - 1, &cuts, &mut memo)
    }

    fn cost(
        left: usize,
        right: usize,
        cuts: &[i32],
        memo: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        if right - left == 1 {
            return 0;
        }
        let key = (left, right);
        if let Some(value) = memo.get(&key) {
            return *value;
        }
        let result = (left + 1..right)
            .map(|mid| {
                Self::cost(left, mid, cuts, memo) + Self::cost(mid, right, cuts, memo) + cuts[right]
                    - cuts[left]
            })
            .min()
            .unwrap();
        memo.insert(key, result);
        result
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(7,vec![1,3,4,5],16)]
    #[case(9,vec![5,6,1,4,2],22)]
    fn case(#[case] n: i32, #[case] cuts: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::min_cost(n, cuts);
        assert_eq!(actual, expected);
    }
}
