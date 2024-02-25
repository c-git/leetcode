//! Solution for https://leetcode.com/problems/greatest-common-divisor-traversal
//! 2709. Greatest Common Divisor Traversal

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

    fn rank(&mut self, x: usize) -> usize {
        let parent = self.find(x);
        self.rank[parent]
    }
}

impl Solution {
    pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
        // Based on https://leetcode.com/problems/greatest-common-divisor-traversal/solutions/3569019/rust-union-join-prime-factorization/

        // handle edge case
        if nums.len() == 1 {
            return true;
        }
        let mut union_find = UnionFind::new(nums.len());
        let mut mp = std::collections::HashMap::<i32, usize>::new();

        for (i, val) in nums.iter().enumerate() {
            let mut a = *val;
            if a == 1 {
                return false;
            }

            if let Some(j) = mp.get(&a) {
                union_find.union(i, *j);
            }
            mp.insert(a, i);

            for b in 2..a {
                if b as i64 * b as i64 > nums[i] as i64 {
                    break;
                }
                if a % b != 0 {
                    continue;
                }

                if let Some(j) = mp.get(&b) {
                    union_find.union(i, *j);
                }
                mp.insert(b, i);
                while a % b == 0 {
                    a /= b;
                }
                if a == 1 {
                    break;
                }
                if let Some(j) = mp.get(&a) {
                    union_find.union(i, *j);
                }
                mp.insert(a, i);
            }
        }

        union_find.rank(0) == nums.len()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,3,6], true)]
    #[case(vec![3,9,5], false)]
    #[case(vec![4,3,12,8], true)]
    #[case(vec![99991; 100_000], true)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: bool) {
        let actual = Solution::can_traverse_all_pairs(nums);
        assert_eq!(actual, expected);
    }
}
