//! Solution for https://leetcode.com/problems/increment-submatrices-by-one
//! 2536. Increment Submatrices by One

impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![0; n as usize]; n as usize];
        for query in queries {
            let [row1, col1, row2, col2] = query[..] else {
                unreachable!("guaranteed by requirements");
            };
            for row in row1..=row2 {
                let row = row as usize;
                for col in col1..=col2 {
                    let col = col as usize;
                    result[row][col] += 1;
                }
            }
        }
        result
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(3, vec![vec![1,1,2,2],vec![0,0,1,1]], vec![vec![1,1,0],vec![1,2,1],vec![0,1,1]])]
    #[case(2, vec![vec![0,0,1,1]], vec![vec![1,1],vec![1,1]])]
    fn case(#[case] n: i32, #[case] queries: Vec<Vec<i32>>, #[case] expected: Vec<Vec<i32>>) {
        let actual = Solution::range_add_queries(n, queries);
        assert_eq!(actual, expected);
    }
}
