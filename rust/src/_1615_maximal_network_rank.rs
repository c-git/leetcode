//! Solution for https://leetcode.com/problems/maximal-network-rank
//! 1615. Maximal Network Rank

impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut result = 0;
        let mut connected = vec![vec![false; n]; n];

        // Populate network
        for road in roads {
            let c1 = road[0] as usize;
            let c2 = road[1] as usize;
            connected[c1][c2] = true;
            connected[c2][c1] = true;
        }

        let road_counts: Vec<i32> = connected
            .iter()
            .map(|x| x.iter().map(|&x| if x { 1 } else { 0 }).sum())
            .collect();

        // Check size of pairs
        for c1 in 0..n {
            for c2 in c1 + 1..n {
                result = result
                    .max(road_counts[c1] + road_counts[c2] - if connected[c1][c2] { 1 } else { 0 });
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
    #[case(4, vec![vec![0,1],vec![0,3],vec![1,2],vec![1,3]], 4)]
    #[case(5, vec![vec![0,1],vec![0,3],vec![1,2],vec![1,3],vec![2,3],vec![2,4]], 5)]
    #[case(8, vec![vec![0,1],vec![1,2],vec![2,3],vec![2,4],vec![5,6],vec![5,7]], 5)]
    fn case(#[case] n: i32, #[case] roads: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::maximal_network_rank(n, roads);
        assert_eq!(actual, expected);
    }
}
