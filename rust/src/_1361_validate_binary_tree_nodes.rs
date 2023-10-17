//! Solution for https://leetcode.com/problems/validate-binary-tree-nodes
//! 1361. Validate Binary Tree Nodes

impl Solution {
    pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
        // Ensure no node has more than one parent and exactly 1 node (the root) doesn't have a parent
        let mut has_parent = vec![false; n as usize];

        for i in 0..n as usize {
            if left_child[i] >= 0 {
                let left = left_child[i] as usize;
                if has_parent[left] {
                    return false; // Two parents
                }
                has_parent[left] = true;
            }
            if right_child[i] >= 0 {
                let right = right_child[i] as usize;
                if has_parent[right] {
                    return false; // Two parents
                }
                has_parent[right] = true;
            }
        }

        dbg!(has_parent)
            .iter()
            .map(|&x| if x { 0 } else { 1 })
            .sum::<u32>()
            == 1
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
