//! Solution for https://leetcode.com/problems/maximal-network-rank
//! 1615. Maximal Network Rank

impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        // Uses idea from editorial comments to sort by number of edges
        let n = n as usize;
        let mut result = None;
        let mut connected = vec![vec![false; n]; n];

        // Populate network
        for road in roads {
            let c1 = road[0] as usize;
            let c2 = road[1] as usize;
            connected[c1][c2] = true;
            connected[c2][c1] = true;
        }

        let mut road_counts: Vec<(usize, i32)> = connected
            .iter()
            .enumerate()
            .map(|(i, x)| (i, x.iter().map(|&x| if x { 1 } else { 0 }).sum()))
            .collect();
        road_counts.sort_unstable_by_key(|(_, x)| -x);

        // Check size of pairs
        for c1 in 0..n {
            for c2 in c1 + 1..n {
                let city1_idx = road_counts[c1].0;
                let city2_idx = road_counts[c2].0;
                let count = road_counts[c1].1 + road_counts[c2].1
                    - if connected[city1_idx][city2_idx] {
                        1
                    } else {
                        0
                    };

                if let Some(val) = result {
                    if val < count {
                        result = Some(count)
                    } else if val > count + 1 {
                        // It can never be bigger because cities are sorted by number of roads
                        break;
                    }
                } else {
                    result = Some(count);
                }
            }
        }

        result.unwrap()
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
