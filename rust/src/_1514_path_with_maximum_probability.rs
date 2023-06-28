//! Solution for https://leetcode.com/problems/path-with-maximum-probability
//! 1514. Path with Maximum Probability

use std::{collections::BinaryHeap, fmt::Debug};

#[derive(Clone, Copy, PartialEq)]
/// Wraps a float and implements Ord but panics if value is NaN
struct FloatNonNAN(f64);
impl Debug for FloatNonNAN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl FloatNonNAN {
    /// Creates a new [Self] with constraint that value cannot be NAN or function panics
    pub fn new(value: f64) -> Self {
        assert!(
            !value.is_nan(),
            "NAN is not allowed as it is not equal to itself and thus invalid for an impl of Ord"
        );
        Self(value)
    }
}
impl Eq for FloatNonNAN {}
impl Ord for FloatNonNAN {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0
            .partial_cmp(&other.0)
            .expect("This should work unless one of them is NAN")
    }
}
impl PartialOrd for FloatNonNAN {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl AsRef<f64> for FloatNonNAN {
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}
impl PartialEq<f64> for FloatNonNAN {
    fn eq(&self, other: &f64) -> bool {
        &self.0 == other
    }
}
impl From<FloatNonNAN> for f64 {
    fn from(value: FloatNonNAN) -> Self {
        value.0
    }
}
impl std::ops::Mul for FloatNonNAN {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        FloatNonNAN::new(self.0 * rhs.0)
    }
}

#[derive(Clone)]
struct Edge {
    dst: usize,
    succ_prob: FloatNonNAN,
}

struct Graph {
    edges: Vec<Vec<Edge>>,
}

impl Graph {
    fn new(size: usize) -> Self {
        Self {
            edges: vec![vec![]; size],
        }
    }

    fn populate(&mut self, edges: Vec<Vec<i32>>, succ_probs: Vec<f64>) {
        debug_assert_eq!(edges.len(), succ_probs.len());
        for (edge, succ_prob) in edges.into_iter().zip(succ_probs.into_iter()) {
            let (vertex1, vertex2) = (edge[0] as usize, edge[1] as usize);
            self.edges[vertex1].push(Edge {
                dst: vertex2,
                succ_prob: FloatNonNAN::new(succ_prob),
            });
            self.edges[vertex2].push(Edge {
                dst: vertex1,
                succ_prob: FloatNonNAN::new(succ_prob),
            });
        }
    }

    fn find_path(&self, start: usize, end: usize) -> f64 {
        let mut heap = BinaryHeap::from([(FloatNonNAN::new(1.0), start)]);
        let mut visited = vec![false; self.edges.len()];
        visited[start] = true;
        while let Some((running_prob, node)) = heap.pop() {
            if node == end {
                return running_prob.into();
            }
            for edge in self.edges[node].iter() {
                if !visited[edge.dst] || edge.dst == end {
                    let new_prob = edge.succ_prob * running_prob;
                    visited[edge.dst] = true;
                    heap.push((new_prob, edge.dst));
                }
            }
        }
        0.0 // Not able to reach end
    }
}

impl Solution {
    pub fn max_probability(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start: i32,
        end: i32,
    ) -> f64 {
        let mut graph = Graph::new(n as usize);
        graph.populate(edges, succ_prob);
        graph.find_path(start as usize, end as usize)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(3, vec![vec![0,1],vec![1,2],vec![0,2]], vec![0.5,0.5,0.2], 0, 2, 0.25)]
    #[case(3, vec![vec![0,1],vec![1,2],vec![0,2]], vec![0.5,0.5,0.3], 0, 2, 0.3)]
    #[case(3, vec![vec![0,1]], vec![0.5], 0, 2, 0.0)]
    fn case(
        #[case] n: i32,
        #[case] edges: Vec<Vec<i32>>,
        #[case] succ_prob: Vec<f64>,
        #[case] start: i32,
        #[case] end: i32,
        #[case] expected: f64,
    ) {
        let actual = Solution::max_probability(n, edges, succ_prob, start, end);
        assert!((actual - expected).abs() < 1e-5, "Assertion failed: actual {actual:.5} but expected {expected:.5}. Diff is more than 1e-5.");
    }
}
