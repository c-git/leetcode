use std::mem::swap;

struct UnionFind {
    components: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            components: (0..n).collect(),
            rank: vec![1; n],
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
                swap(&mut root_x, &mut root_y)
            }
            self.rank[root_y] += self.rank[root_x];
            self.components[root_x] = root_y;
        }
    }

    fn is_same_set(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
}

impl Solution {
    // Source: Based on faster submission on leetcode (Updated to include join by rank after reading editorial)
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut uf = UnionFind::new(n as usize);
        for edge in edges {
            uf.union(edge[0] as usize, edge[1] as usize)
        }
        uf.is_same_set(source as usize, destination as usize)
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 3;
        let edges = [[0, 1], [1, 2], [2, 0]];
        let source = 0;
        let destination = 2;
        let edges = edges.into_iter().map(|x| x.into()).collect();
        let expected = true;
        let actual = Solution::valid_path(n, edges, source, destination);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let n = 6;
        let edges = [[0, 1], [0, 2], [3, 5], [5, 4], [4, 3]];
        let source = 0;
        let destination = 5;
        let edges = edges.into_iter().map(|x| x.into()).collect();
        let expected = false;
        let actual = Solution::valid_path(n, edges, source, destination);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case3() {
        let n = 1;
        let edges: [[i32; 2]; 0] = [];
        let source = 0;
        let destination = 0;
        let edges = edges.into_iter().map(|x| x.into()).collect();
        let expected = true;
        let actual = Solution::valid_path(n, edges, source, destination);
        assert_eq!(actual, expected);
    }
}
