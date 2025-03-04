//! Solution for https://leetcode.com/problems/most-profitable-path-in-a-tree
//! 2467. Most Profitable Path in a Tree

use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap, VecDeque},
};

/// New type added to prevent using index in place of a distance by accident
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Distance(usize);
type NodeIndex = usize;

struct Graph {
    edges: Vec<BTreeSet<usize>>,
}

struct MinHeap {
    max_heap: BinaryHeap<Reverse<(Distance, NodeIndex)>>,
}

impl Distance {
    const MAX: Self = Self(usize::MAX);
    #[must_use]
    fn succ(&self) -> Self {
        Self(self.0 + 1)
    }
}

impl MinHeap {
    fn new() -> Self {
        Self {
            max_heap: Default::default(),
        }
    }

    fn push(&mut self, item: (Distance, NodeIndex)) {
        self.max_heap.push(Reverse(item));
    }

    fn pop(&mut self) -> Option<(Distance, NodeIndex)> {
        Some(self.max_heap.pop()?.0)
    }
}

impl Graph {
    fn new(edges: Vec<[usize; 2]>) -> Self {
        let mut result = Self {
            edges: vec![Default::default(); edges.len() + 1],
        };
        for edge in edges {
            result.add_edge(edge);
        }
        result
    }

    fn add_edge(&mut self, edge: [usize; 2]) {
        let [x, y] = edge;
        self.edges[x].insert(y);
        self.edges[y].insert(x);
    }

    fn len(&self) -> usize {
        self.edges.len()
    }

    /// Returns the shortest unweighted path from dst to src including endpoints
    /// if a path exists between them
    ///
    /// Output will always start with `dst` and end with `src`
    fn path(&self, src: usize, dst: usize) -> Vec<usize> {
        // Using Dijkstra's algorithm with constant edge weight of 1
        let mut distances = vec![Distance::MAX; self.len()];
        let mut min_distance_heap = MinHeap::new();
        min_distance_heap.push((Distance(0), src));
        distances[src] = Distance(0);
        let mut visited = vec![false; self.len()];
        let mut previous = vec![None; self.len()];
        while let Some((distance, node_index)) = min_distance_heap.pop() {
            if node_index == dst {
                // We don't care about the rest of the graph we found the destination
                break;
            }
            debug_assert_eq!(
                distance, distances[node_index],
                "When remove this should always be the lowest distance for this node"
            );
            debug_assert!(
                !visited[node_index],
                "Should never visit the same node twice"
            );
            visited[node_index] = true;
            let alt = distance.succ();
            for &neighbour in self.edges[node_index].iter() {
                if visited[neighbour] {
                    continue;
                }
                if alt < distances[neighbour] {
                    distances[neighbour] = alt;
                    min_distance_heap.push((alt, neighbour));
                    previous[neighbour] = Some(node_index);
                }
            }
        }

        // Extract path found
        let mut result = vec![dst];
        let mut next_opt = previous[dst];
        while let Some(curr) = next_opt {
            result.push(curr);
            next_opt = previous[curr];
        }
        debug_assert_eq!(result.last().unwrap(), &src);
        result
    }
}

impl Solution {
    /// Using plan suggested by hints for question
    /// - Find bob's path
    /// - Use bob's path to update the amounts that alice can receive
    /// - Use BFS to calculate cost to each leaf and return highest value
    pub fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, mut amount: Vec<i32>) -> i32 {
        // Build Graph
        let graph = Graph::new(
            edges
                .into_iter()
                .map(|x| [x[0] as usize, x[1] as usize])
                .collect(),
        );

        // Update amounts based on Bob's path
        Self::update_amounts(&graph, bob as _, &mut amount);

        // Use BFS to find best path to a leaf which is a node that has a no outbound
        // path that hasn't been traversed
        let mut result = i32::MIN;
        let mut queue = VecDeque::new();
        queue.push_back((0, amount[0]));
        let mut visited = vec![false; graph.len()];
        while let Some((curr_node_idx, curr_profit)) = queue.pop_front() {
            debug_assert!(!visited[curr_node_idx]);
            visited[curr_node_idx] = true;
            let mut is_leaf = true;
            for &neighbour in graph.edges[curr_node_idx].iter() {
                if visited[neighbour] {
                    continue;
                }
                is_leaf = false;
                queue.push_back((neighbour, curr_profit + amount[neighbour]));
            }
            if is_leaf {
                result = result.max(curr_profit);
            }
        }
        result
    }

    fn update_amounts(graph: &Graph, bob: usize, amount: &mut [i32]) {
        let bob_path = graph.path(bob, 0);
        let should_split_half = bob_path.len() % 2 == 1;
        let mut start = bob_path.len() / 2;
        if should_split_half {
            amount[bob_path[start]] /= 2;
            start += 1;
        }
        for &node_idx in bob_path.iter().skip(start) {
            amount[node_idx] = 0;
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
    #[case(vec![vec![0,1],vec![1,2],vec![1,3],vec![3,4]], 3, vec![-2,4,2,-4,6], 6)]
    #[case(vec![vec![0,1]], 1, vec![-7280,2350], -7280)]
    fn case(
        #[case] edges: Vec<Vec<i32>>,
        #[case] bob: i32,
        #[case] amount: Vec<i32>,
        #[case] expected: i32,
    ) {
        let actual = Solution::most_profitable_path(edges, bob, amount);
        assert_eq!(actual, expected);
    }
}
