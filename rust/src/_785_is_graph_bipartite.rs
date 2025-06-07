//! Solution for https://leetcode.com/problems/is-graph-bipartite
//! 785. Is Graph Bipartite?

struct UnionFind {
    nodes: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        Self {
            nodes: (0..size).collect(),
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.nodes[x] == x {
            x
        } else {
            let root = self.find(self.nodes[x]);
            self.nodes[x] = root;
            root
        }
    }
    fn merge(&mut self, x: usize, y: usize) {
        let x_root = self.find(x);
        let y_root = self.find(y);
        self.nodes[y_root] = x_root;
    }
}

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let mut uf = UnionFind::new(graph.len());
        for (node, neighbours) in graph.into_iter().enumerate() {
            if neighbours.is_empty() {
                continue;
            }
            let other_root = uf.find(neighbours[0] as usize);
            for neighbour in neighbours {
                uf.merge(other_root, neighbour as usize);
                if uf.find(node) == uf.find(other_root) {
                    return false;
                }
            }
        }
        true
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,2,3],vec![0,2],vec![0,1,3],vec![0,2]], false)]
    #[case(vec![vec![1,3],vec![0,2],vec![1,3],vec![0,2]], true)]
    #[case(vec![vec![1], vec![0, 3], vec![3], vec![1, 2]], true)]
    #[case(vec![vec![3], vec![2, 4], vec![1], vec![0, 4], vec![1, 3]], true)]
    #[case(vec![vec![3], vec![2, 4], vec![1], vec![0, 4], vec![1, 3]], true)]
    #[case(vec![vec![3],vec![2,3],vec![1,3],vec![0,2,1]], false)]
    fn case(#[case] graph: Vec<Vec<i32>>, #[case] expected: bool) {
        let actual = Solution::is_bipartite(graph);
        assert_eq!(actual, expected);
    }
}
