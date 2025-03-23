//! Solution for https://leetcode.com/problems/number-of-ways-to-arrive-at-destination
//! 1976. Number of Ways to Arrive at Destination

const MODULUS: usize = 1_000_000_007;

#[derive(Clone, Copy)]
struct Edge {
    /// The other end of the road
    dst: usize,
    time: usize,
}

struct Graph {
    edges: Vec<Vec<Edge>>,
}

impl Graph {
    fn new(size: usize) -> Self {
        Self {
            edges: vec![Vec::new(); size],
        }
    }

    fn add_edge(&mut self, edge_info: Vec<i32>) {
        let node1 = edge_info[0] as usize;
        let node2 = edge_info[1] as usize;
        let time = edge_info[2] as usize;
        self.edges[node1].push(Edge { dst: node2, time });
        self.edges[node2].push(Edge { dst: node1, time });
    }
}

type Time = usize;
type Intersection = usize;

struct MinHeap {
    max_heap: std::collections::BinaryHeap<std::cmp::Reverse<(Time, Intersection)>>,
}

impl MinHeap {
    fn new() -> Self {
        Self {
            max_heap: Default::default(),
        }
    }

    fn push(&mut self, value: (Time, Intersection)) {
        self.max_heap.push(std::cmp::Reverse(value));
    }

    fn pop(&mut self) -> Option<(Time, Intersection)> {
        self.max_heap.pop().map(|x| x.0)
    }
}

impl Solution {
    /// After reading editorial
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut shortest_paths_times = vec![usize::MAX; n];
        let mut path_count = vec![0; n];
        shortest_paths_times[0] = 0; // Starting point
        path_count[0] = 1; // Exactly 1 way to get to starting point from the starting point

        // Build Graph
        let mut graph = Graph::new(n);
        for road in roads {
            graph.add_edge(road);
        }

        // Use Dijkstra's algorithm to get the number of shortest paths available
        let mut min_heap = MinHeap::new();
        min_heap.push((0, 0));
        while let Some((time, intersection)) = min_heap.pop() {
            if time > shortest_paths_times[intersection] {
                // Discard larger value for known path
                continue;
            }
            for edge in graph.edges[intersection].iter().copied() {
                let new_time = time + edge.time;
                match new_time.cmp(&shortest_paths_times[edge.dst]) {
                    std::cmp::Ordering::Less => {
                        shortest_paths_times[edge.dst] = new_time;
                        path_count[edge.dst] = path_count[intersection];
                        min_heap.push((new_time, edge.dst));
                    }
                    std::cmp::Ordering::Equal => {
                        path_count[edge.dst] =
                            (path_count[edge.dst] + path_count[intersection]) % MODULUS;
                    }
                    std::cmp::Ordering::Greater => {} // Do not update this is a slower path
                }
            }
        }

        path_count[n - 1] as _
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
