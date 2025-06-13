//! Solution for https://leetcode.com/problems/cheapest-flights-within-k-stops
//! 787. Cheapest Flights Within K Stops

impl Solution {
    /// Based on https://www.youtube.com/watch?v=5eIK3zUdYmE
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut distances = vec![i32::MAX; n as usize];
        distances[src as usize] = 0;
        let mut temp_distances = distances.clone();
        for _ in 0..=k {
            for flight in flights.iter() {
                let from = flight[0] as usize;
                let to = flight[1] as usize;
                let cost = flight[2];
                temp_distances[to] = temp_distances[to].min(distances[from].saturating_add(cost));
            }
            distances.clone_from(&temp_distances);
        }

        if distances[dst as usize] < i32::MAX {
            distances[dst as usize]
        } else {
            // Not found
            -1
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(4, vec![vec![0,1,100],vec![1,2,100],vec![2,0,100],vec![1,3,600],vec![2,3,200]], 0, 3, 1, 700)]
    #[case(3, vec![vec![0,1,100],vec![1,2,100],vec![0,2,500]], 0, 2, 1, 200)]
    #[case(3, vec![vec![0,1,100],vec![1,2,100],vec![0,2,500]], 0, 2, 0, 500)]
    #[case(4, vec![vec![0,1,1],vec![0,2,5],vec![1,2,1],vec![2,3,1]], 0, 3, 1, 6)]
    #[case(11, vec![vec![0,3,3],vec![3,4,3],vec![4,1,3],vec![0,5,1],vec![5,1,100],vec![0,6,2],vec![6,1,100],vec![0,7,1],vec![7,8,1],vec![8,9,1],vec![9,1,1],vec![1,10,1],vec![10,2,1],vec![1,2,100]], 0, 2, 4, 11)]
    fn case(
        #[case] n: i32,
        #[case] flights: Vec<Vec<i32>>,
        #[case] src: i32,
        #[case] dst: i32,
        #[case] k: i32,
        #[case] expected: i32,
    ) {
        let actual = Solution::find_cheapest_price(n, flights, src, dst, k);
        assert_eq!(actual, expected);
    }
}
