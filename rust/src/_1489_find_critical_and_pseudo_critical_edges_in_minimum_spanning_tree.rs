//! Solution for https://leetcode.com/problems/find-critical-and-pseudo-critical-edges-in-minimum-spanning-tree
//! 1489. Find Critical and Pseudo-Critical Edges in Minimum Spanning Tree

use std::mem;

struct UnionFind {
    components: Vec<usize>,
    rank: Vec<usize>,
    max_rank: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            components: (0..n).collect(),
            rank: vec![1; n],
            max_rank: 1,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if x != self.components[x] {
            self.components[x] = self.find(self.components[x]);
        }
        self.components[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let mut root_x = self.find(x);
        let mut root_y = self.find(y);
        if root_x != root_y {
            if self.rank[root_x] > self.rank[root_y] {
                mem::swap(&mut root_x, &mut root_y)
            }
            self.rank[root_y] += self.rank[root_x];
            self.components[root_x] = root_y;
            self.max_rank = self.max_rank.max(self.rank[root_y]);
        }
    }

    fn is_same_set(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    fn is_one_set_only(&self) -> bool {
        self.max_rank == self.components.len()
    }
}

#[derive(PartialEq, Eq)]
struct Edge {
    u: usize,
    v: usize,
    weight: i32,
    original_idx: i32,
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}

enum SpecialEdge {
    No,
    Ignore(usize),
    MustUse(usize),
}

struct KruskalResult {
    weight: i32,
    is_connected: bool,
}

impl Solution {
    pub fn find_critical_and_pseudo_critical_edges(
        n: i32,
        mut edges: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        // After reading editorial
        let n = n as usize;
        let mut result_critical = vec![];
        let mut result_pseudo_critical = vec![];

        // Convert all edges into struct representation for code readability
        let mut edges: Vec<Edge> = edges
            .into_iter()
            .enumerate()
            .map(|(i, x)| Edge {
                u: x[0] as usize,
                v: x[1] as usize,
                weight: x[2],
                original_idx: i as i32,
            })
            .collect();
        edges.sort_unstable();

        let best = Self::kruskal_algorithm(&edges, n, SpecialEdge::No);
        debug_assert!(best.is_connected);

        for (i, edge) in edges.iter().enumerate() {
            let ignore = Self::kruskal_algorithm(&edges, n, SpecialEdge::Ignore(i));
            if !ignore.is_connected || ignore.weight > best.weight {
                result_critical.push(edge.original_idx);
                continue;
            }

            let force = Self::kruskal_algorithm(&edges, n, SpecialEdge::MustUse(i));
            if force.weight == best.weight {
                result_pseudo_critical.push(edge.original_idx);
            }
        }

        vec![result_critical, result_pseudo_critical]
    }

    fn kruskal_algorithm(edges: &[Edge], n: usize, special: SpecialEdge) -> KruskalResult {
        let mut uf = UnionFind::new(n);
        let mut result_weight = 0;

        let ignore_edge = match special {
            SpecialEdge::No => None,
            SpecialEdge::Ignore(idx) => Some(idx),
            SpecialEdge::MustUse(idx) => {
                let edge = &edges[idx];
                uf.union(edge.u, edge.v);
                result_weight += edge.weight;
                Some(idx) // Already inserted ignore going forward
            }
        };

        for (i, edge) in edges.iter().enumerate() {
            if Some(i) == ignore_edge {
                continue; // Skip this edge
            }
            if !uf.is_same_set(edge.u, edge.v) {
                result_weight += edge.weight;
                uf.union(edge.u, edge.v);
            }
        }

        KruskalResult {
            weight: result_weight,
            is_connected: uf.is_one_set_only(),
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
    #[case(5, vec![vec![0,1,1],vec![1,2,1],vec![2,3,2],vec![0,3,2],vec![0,4,3],vec![3,4,3],vec![1,4,6]], vec![vec![0,1],vec![2,3,4,5]])]
    #[case(4, vec![vec![0,1,1],vec![1,2,1],vec![2,3,1],vec![0,3,1]], vec![vec![],vec![0,1,2,3]])]
    fn case(#[case] n: i32, #[case] edges: Vec<Vec<i32>>, #[case] mut expected: Vec<Vec<i32>>) {
        let mut actual = Solution::find_critical_and_pseudo_critical_edges(n, edges);
        actual.iter_mut().for_each(|x| x.sort_unstable());
        expected.iter_mut().for_each(|x| x.sort_unstable());
        assert_eq!(actual, expected);
    }
}
