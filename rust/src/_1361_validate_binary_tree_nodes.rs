//! Solution for https://leetcode.com/problems/validate-binary-tree-nodes
//! 1361. Validate Binary Tree Nodes

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
                std::mem::swap(&mut root_x, &mut root_y)
            }
            self.rank[root_y] += self.rank[root_x];
            self.components[root_x] = root_y;
        }
    }

    fn find_rank(&mut self, x: usize) -> usize {
        let component = self.find(x);
        self.rank[component]
    }

    fn is_same_set(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
}

impl Solution {
    pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
        // Keep each node in a separate set until it is assigned a parent, then check that there is only one set at the end
        let mut has_parent = vec![false; n as usize];
        let mut uf = UnionFind::new(n as usize);

        for i in 0..n as usize {
            if left_child[i] >= 0 {
                let left = left_child[i] as usize;
                if has_parent[left] {
                    return false; // Two parents
                }
                has_parent[left] = true;
                if uf.is_same_set(i, left) {
                    return false; // Cycle detected
                }
                uf.union(i, left);
            }
            if right_child[i] >= 0 {
                let right = right_child[i] as usize;
                if has_parent[right] {
                    return false; // Two parents
                }
                has_parent[right] = true;
                if uf.is_same_set(i, right) {
                    return false; // Cycle detected
                }
                uf.union(i, right);
            }
        }

        // Check if all nodes are in one set
        uf.find_rank(0) == n as usize
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(4, vec![1,-1,3,-1], vec![2,-1,-1,-1], true)]
    #[case(4, vec![1,-1,3,-1], vec![2,3,-1,-1], false)]
    #[case(2, vec![1,0], vec![-1,-1], false)]
    #[case(2, vec![-1,-1], vec![-1,-1], false)]
    #[case(4, vec![1,0,3,-1], vec![-1,-1,-1,-1], false)]
    fn case(
        #[case] n: i32,
        #[case] left_child: Vec<i32>,
        #[case] right_child: Vec<i32>,
        #[case] expected: bool,
    ) {
        let actual = Solution::validate_binary_tree_nodes(n, left_child, right_child);
        assert_eq!(actual, expected);
    }
}
