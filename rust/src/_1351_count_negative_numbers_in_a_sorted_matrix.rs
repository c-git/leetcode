impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid[0].len();
        let mut result = 0;
        for row in grid {
            result += (n - row.partition_point(|x| x >= &0)) as i32;
        }
        result
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![4,3,2,-1],vec![3,2,1,-1],vec![1,1,-1,-2],vec![-1,-1,-2,-3]], 8)]
    #[case(vec![vec![3,2],vec![1,0]], 0)]
    fn case(#[case] input: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::count_negatives(input);
        assert_eq!(actual, expected);
    }
}
