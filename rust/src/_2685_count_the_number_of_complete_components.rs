//! Solution for https://leetcode.com/problems/count-the-number-of-complete-components
//! 2685. Count the Number of Complete Components

struct UnionFind {
    nodes: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        Self {
            nodes: (0..size).collect(),
            rank: vec![1; size],
        }
    }

    fn root(&mut self, idx: usize) -> usize {
        if self.nodes[idx] != idx {
            self.nodes[idx] = self.root(self.nodes[idx]);
        }
        self.nodes[idx]
    }

    fn join(&mut self, edge: [usize; 2]) {
        let [idx1, idx2] = edge;
        let mut root1 = self.root(idx1);
        let mut root2 = self.root(idx2);
        if root1 == root2 {
            return;
        }
        if self.rank[root1] < self.rank[root2] {
            std::mem::swap(&mut root1, &mut root2);
        }
        let rank2 = self.rank[root2];

        self.nodes[root2] = root1;
        self.rank[root1] += rank2;
    }

    fn rank(&mut self, idx: usize) -> usize {
        let root = self.root(idx);
        self.rank[root]
    }
}

struct Graph {
    edges: Vec<Vec<usize>>,
}

impl Graph {
    fn new(size: usize) -> Self {
        Self {
            edges: vec![Vec::new(); size],
        }
    }

    fn add_edge(&mut self, edge: [usize; 2]) {
        let [node1, node2] = edge;
        self.edges[node1].push(node2);
        self.edges[node2].push(node1);
    }
}

impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let n = n as usize;

        // Build Graph and UnionFind
        let mut union_find = UnionFind::new(n);
        let mut graph = Graph::new(n);
        let edge_iter = edges.into_iter().map(|edge| {
            debug_assert_eq!(edge.len(), 2);
            [edge[0] as usize, edge[1] as usize]
        });
        for edge in edge_iter {
            union_find.join(edge);
            graph.add_edge(edge);
        }

        // Ensure all nodes have a number of edges equal to the number of vertices in their component
        let mut visited = vec![false; n];
        for i in 0..n {
            if visited[i] {
                continue;
            }
            visited[i] = true;
            let component_size = union_find.rank(i);
            // Ensure all components have the correct number of edges
            let mut is_invalid_found = graph.edges[i].len() != component_size - 1;
            for neighbour in graph.edges[i].iter().copied() {
                visited[neighbour] = true;
                is_invalid_found =
                    is_invalid_found || graph.edges[neighbour].len() != component_size - 1;
            }
            if !is_invalid_found {
                result += 1;
            }
        }
        result
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(6, vec![vec![0,1],vec![0,2],vec![1,2],vec![3,4]], 3)]
    #[case(6, vec![vec![0,1],vec![0,2],vec![1,2],vec![3,4],vec![3,5]], 1)]
    fn case(#[case] n: i32, #[case] edges: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::count_complete_components(n, edges);
        assert_eq!(actual, expected);
    }
}
