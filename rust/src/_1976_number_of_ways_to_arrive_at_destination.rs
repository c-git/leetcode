//! Solution for https://leetcode.com/problems/number-of-ways-to-arrive-at-destination
//! 1976. Number of Ways to Arrive at Destination

use std::collections::BinaryHeap;

const MODULO: usize = 1_000_000_007;
type Time = usize;
type Intersection = usize;

#[derive(Default)]
struct MinHeap {
    max_heap: BinaryHeap<std::cmp::Reverse<(Time, Intersection)>>,
}

impl MinHeap {
    fn push(&mut self, time: Time, intersection: Intersection) {
        self.max_heap.push(std::cmp::Reverse((time, intersection)));
    }

    fn pop(&mut self) -> Option<(Time, Intersection)> {
        let std::cmp::Reverse(result) = self.max_heap.pop()?;
        Some(result)
    }
}

struct Graph {
    roads: Vec<Vec<(Intersection, Time)>>,
}

impl Graph {
    fn new(size: usize) -> Self {
        Self {
            roads: vec![vec![]; size],
        }
    }

    fn add_road(&mut self, side1: Intersection, side2: Intersection, time: Time) {
        self.roads[side1].push((side2, time));
        self.roads[side2].push((side1, time));
    }
}

impl Solution {
    /// After reading over previous solution that was based on the editorial
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;

        let mut path_count = vec![0; n];
        let mut shortest_time = vec![usize::MAX; n];
        path_count[0] = 1; // One way to start
        shortest_time[0] = 0; // No time required to get to start

        // Build graph
        let mut graph = Graph::new(n);
        for road in roads {
            graph.add_road(road[0] as usize, road[1] as usize, road[2] as usize);
        }

        let mut heap = MinHeap::default();
        heap.push(0, 0);

        while let Some((time, src_intersection)) = heap.pop() {
            println!("{time} {src_intersection}");
            if time > shortest_time[src_intersection] {
                // Skip slow routes to known intersections
                continue;
            }
            for (dst, road_time) in graph.roads[src_intersection].iter().copied() {
                let new_time = time + road_time;
                match new_time.cmp(&shortest_time[dst]) {
                    std::cmp::Ordering::Less => {
                        // New path forget about old ones
                        shortest_time[dst] = new_time;
                        path_count[dst] = path_count[src_intersection];
                        heap.push(new_time, dst);
                    }
                    std::cmp::Ordering::Equal => {
                        // Increment
                        path_count[dst] = (path_count[dst] + path_count[src_intersection]) % MODULO;
                    }
                    std::cmp::Ordering::Greater => {} // Slower ignore
                }
            }
            println!("{shortest_time:?} {path_count:?}");
        }

        (*path_count.last().unwrap()) as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(7, vec![vec![0,6,7],vec![0,1,2],vec![1,2,3],vec![1,3,3],vec![6,3,3],vec![3,5,1],vec![6,5,1],vec![2,5,1],vec![0,4,5],vec![4,6,2]], 4)]
    #[case(2, vec![vec![1,0,10]], 1)]
    fn case(#[case] n: i32, #[case] roads: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::count_paths(n, roads);
        assert_eq!(actual, expected);
    }
}
