//! Solution for https://leetcode.com/problems/cheapest-flights-within-k-stops
//! 787. Cheapest Flights Within K Stops

impl Solution {
    /// Based on https://www.youtube.com/watch?v=5eIK3zUdYmE
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut costs = vec![None; n as usize];
        costs[src as usize] = Some(0);
        let mut temp_costs = costs.clone();
        for _ in 0..=k {
            for flight in flights.iter() {
                let &[from, to, price] = flight.as_slice() else {
                    unreachable!("guaranteed to be 3 long")
                };
                if let Some(from_cost) = costs[from as usize] {
                    let new_possibility = from_cost + price;
                    temp_costs[to as usize] = temp_costs[to as usize]
                        .map(|x| x.min(new_possibility))
                        .or(Some(new_possibility));
                }
            }
            costs = temp_costs.clone();
        }
        costs[dst as usize].unwrap_or(-1)
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
