//! Solution for https://leetcode.com/problems/cheapest-flights-within-k-stops
//! 787. Cheapest Flights Within K Stops

use std::{cmp::Reverse, collections::BinaryHeap};

struct Graph {
    nodes: Vec<Vec<(usize, i32)>>,
}

#[derive(Default)]
struct MinHeap {
    max_heap: BinaryHeap<Reverse<(i32, i32, usize)>>,
}

impl MinHeap {
    fn push(&mut self, val: (i32, i32, usize)) {
        self.max_heap.push(Reverse(val));
    }
    fn pop(&mut self) -> Option<(i32, i32, usize)> {
        self.max_heap.pop().map(|x| x.0)
    }
}

impl Solution {
    // Performance fixed using https://www.youtube.com/watch?v=vWgoPTvQ3Rw
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let n = n as usize;
        let mut graph = Graph {
            nodes: vec![vec![]; n],
        };
        for flight in flights {
            debug_assert_eq!(flight.len(), 3);
            let (from, to, price) = (flight[0], flight[1], flight[2]);
            graph.nodes[from as usize].push((to as usize, price));
        }
        let src = src as usize;
        let dst = dst as usize;
        let mut max_stops_left = vec![0; n + 1];
        let mut min_cost = vec![i32::MAX; n + 1];
        min_cost[src] = 0;
        let mut heap = MinHeap::default();
        heap.push((0, k + 1, src));
        while let Some((cost, curr_stops_left, node_idx)) = heap.pop() {
            if node_idx == dst {
                if curr_stops_left <= k {
                    return cost;
                } else {
                    continue;
                }
            }
            if curr_stops_left == 0 {
                // Unable to use this path as we cannot stop here because we have no stops left
                continue;
            }

            let next_stops_left = curr_stops_left - 1;
            for (neighbour, price) in graph.nodes[node_idx].iter().cloned() {
                let next_cost = cost + price;
                let mut should_try = false;
                if min_cost[neighbour] > next_cost {
                    min_cost[neighbour] = next_cost;
                    should_try = true;
                }
                if next_stops_left > max_stops_left[neighbour] {
                    max_stops_left[neighbour] = next_stops_left;
                    should_try = true;
                }
                if should_try {
                    heap.push((next_cost, next_stops_left, neighbour));
                }
            }
        }

        // Not able to reach destination with required number of stops
        -1
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
    #[case(3, vec![vec![0,1,1],vec![0,2,5],vec![1,2,1],vec![2,3,1]], 0, 3, 1, 6)]
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
