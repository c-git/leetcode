//! Solution for https://leetcode.com/problems/paths-in-matrix-whose-sum-is-divisible-by-k
//! 2435. Paths in Matrix Whose Sum Is Divisible by K

impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let empty: Vec<Vec<Option<i32>>> = vec![vec![None; k as usize]; grid[0].len()]; // Used to have a fast way to get an empty row for DP
        let mut prev = empty.clone();
        let mut curr = empty.clone();

        // Seed with starting from 0 for 0,0
        prev[0][0] = Some(1);

        for row in grid {
            for (i, value) in row.into_iter().enumerate() {
                // Try from above
                for (remainder, ways) in prev[i].iter().enumerate() {
                    if let Some(ways) = ways {
                        let index = (value as usize + remainder) % k as usize;
                        let curr = curr[i][index].get_or_insert_default();
                        *curr += ways;
                        *curr %= MOD;
                    }
                }

                // Try from left
                if i > 0 {
                    for remainder in 0..k as usize {
                        if let Some(ways) = curr[i - 1][remainder] {
                            let index = (value as usize + remainder) % k as usize;
                            let curr = curr[i][index].get_or_insert_default();
                            *curr += ways;
                            *curr %= MOD;
                        }
                    }
                }
            }
            std::mem::swap(&mut prev, &mut curr);
            curr = empty.clone();
        }

        prev.last().unwrap().first().unwrap().unwrap_or_default()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![5,2,4],vec![3,0,5],vec![0,7,2]], 3, 2)]
    #[case(vec![vec![0,0]], 5, 1)]
    #[case(vec![vec![7,3,4,9],vec![2,3,6,2],vec![2,3,7,0]], 1, 10)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::number_of_paths(grid, k);
        assert_eq!(actual, expected);
    }
}
