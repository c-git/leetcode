//! Solution for https://leetcode.com/problems/course-schedule
//! 207. Course Schedule

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

    fn is_same_set(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
}

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        // Use a union find to store courses that share a dependency. If we come to add
        // a course and it is going to be added to the same set then we found a cycle
        // and it's not possible to finish

        let mut uf = UnionFind::new(num_courses as _);
        for pre_req in prerequisites {
            let (x, y) = (pre_req[0] as _, pre_req[1] as _);
            if uf.is_same_set(x, y) {
                return false;
            }
            uf.union(x, y);
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
    #[case(2, vec![vec![1,0]], true)]
    #[case(2, vec![vec![1,0],vec![0,1]], false)]
    fn case(
        #[case] num_courses: i32,
        #[case] prerequisites: Vec<Vec<i32>>,
        #[case] expected: bool,
    ) {
        let actual = Solution::can_finish(num_courses, prerequisites);
        assert_eq!(actual, expected);
    }
}
