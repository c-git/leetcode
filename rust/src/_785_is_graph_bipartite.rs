use std::mem::swap;

struct UnionFind {
    components: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            components: (0..n).collect(),
            rank: vec![1; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if x != self.components[x] {
            self.components[x] = self.find(self.components[x]);
        }
        self.components[x]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let mut root_x = self.find(x);
        let mut root_y = self.find(y);
        if root_x != root_y {
            if self.rank[root_x] > self.rank[root_y] {
                swap(&mut root_x, &mut root_y)
            }
            self.rank[root_y] += self.rank[root_x];
            self.components[root_x] = root_y;
        }
    }

    pub fn is_same_set(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
}

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let mut uf = UnionFind::new(graph.len());

        for (node, edges) in graph.iter().enumerate() {
            // Ensure the node is not in the same set as any of it's edges and ensure it's edges are all in the same set
            let mut iter = edges.iter();
            if let Some(first_edge) = iter.next() {
                let edge_set = uf.find(*first_edge as usize);
                for edge in iter {
                    uf.union(edge_set, *edge as usize);
                }
                if uf.is_same_set(node, edge_set) {
                    return false;
                }
            }
        }
        true
    }
}

struct Solution;

#[test]
fn case1() {
    let graph = vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]];
    let expected = false;

    let actual = Solution::is_bipartite(graph);
    assert_eq!(actual, expected);
}

#[test]
fn case2() {
    let graph = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];
    let expected = true;

    let actual = Solution::is_bipartite(graph);
    assert_eq!(actual, expected);
}

#[test]
fn case3() {
    let graph = vec![vec![1], vec![0, 3], vec![3], vec![1, 2]];
    let expected = true;

    let actual = Solution::is_bipartite(graph);
    assert_eq!(actual, expected);
}

#[test]
fn case4() {
    let graph = vec![vec![3], vec![2, 4], vec![1], vec![0, 4], vec![1, 3]];
    let expected = true;

    let actual = Solution::is_bipartite(graph);
    assert_eq!(actual, expected);
}
