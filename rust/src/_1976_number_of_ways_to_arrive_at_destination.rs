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

impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut result = 0;
        let mut best_time = usize::MAX;
        let mut visited = vec![false; n];
        let mut graph = Graph::new(n);
        for road in roads {
            graph.add_edge(road);
        }
        Self::dfs(
            0,
            n - 1,
            0,
            &graph,
            &mut visited,
            &mut best_time,
            &mut result,
        );
        result as _
    }

    fn dfs(
        src: usize,
        dst: usize,
        travel_time: usize,
        graph: &Graph,
        visited: &mut [bool],
        best_time: &mut usize,
        result: &mut usize,
    ) {
        if src == dst {
            match travel_time.cmp(best_time) {
                std::cmp::Ordering::Less => {
                    *result = 1;
                    *best_time = travel_time;
                }
                std::cmp::Ordering::Equal => *result = (*result + 1) % MODULUS,
                std::cmp::Ordering::Greater => {}
            }
            return;
        }
        if visited[src] || travel_time > *best_time {
            return;
        }
        visited[src] = true;
        for edge in graph.edges[src].iter().copied() {
            Self::dfs(
                edge.dst,
                dst,
                travel_time + edge.time,
                graph,
                visited,
                best_time,
                result,
            );
        }
        visited[src] = false;
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
