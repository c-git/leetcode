struct UnionFind {
    components: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            components: (0..n).collect(),
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if x == self.components[x] {
            return x;
        }
        let parent = self.find(self.components[x]);
        self.components[x] = parent;
        parent
    }

    fn union(&mut self, a: usize, b: usize) {
        let parent_a = self.find(a);
        let parent_b = self.find(b);
        self.components[parent_a] = self.components[parent_b];
    }

    fn is_same_set(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
}

impl Solution {
    // Source: Based on faster submission on leetcode
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
