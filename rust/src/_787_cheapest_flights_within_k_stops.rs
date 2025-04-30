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
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut graph = Graph {
            nodes: vec![vec![]; n as usize],
        };
        for flight in flights {
            debug_assert_eq!(flight.len(), 3);
            let (from, to, price) = (flight[0], flight[1], flight[2]);
            graph.nodes[from as usize].push((to as usize, price));
        }
        let src = src as usize;
        let dst = dst as usize;
        let mut heap = MinHeap::default();
        heap.push((0, k + 1, src));
        while let Some((cost, stops_left, node_idx)) = heap.pop() {
            if node_idx == dst {
                if stops_left <= k {
                    return cost;
                } else {
                    continue;
                }
            }
            if stops_left == 0 {
                // Unable to use this path as we cannot stop here
                continue;
            }

            for (neighbour, price) in graph.nodes[node_idx].iter().cloned() {
                heap.push((cost + price, stops_left - 1, neighbour));
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
