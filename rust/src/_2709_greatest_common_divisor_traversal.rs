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

    fn is_full_rank(&mut self) -> bool {
        dbg!(&self.rank);
        let parent = self.find(0);
        self.rank[parent] == self.rank.len()
    }
}

/// Binary GCD algorithm taken from https://en.wikipedia.org/wiki/Binary_GCD_algorithm
pub fn gcd(mut u: u64, mut v: u64) -> u64 {
    // Base cases: gcd(n, 0) = gcd(0, n) = n
    if u == 0 {
        return v;
    } else if v == 0 {
        return u;
    }

    // Using identities 2 and 3:
    // gcd(2ⁱ u, 2ʲ v) = 2ᵏ gcd(u, v) with u, v odd and k = min(i, j)
    // 2ᵏ is the greatest power of two that divides both 2ⁱ u and 2ʲ v
    let i = u.trailing_zeros();
    u >>= i;
    let j = v.trailing_zeros();
    v >>= j;
    let k = std::cmp::min(i, j);

    loop {
        // u and v are odd at the start of the loop
        debug_assert!(u % 2 == 1, "u = {} should be odd", u);
        debug_assert!(v % 2 == 1, "v = {} should be odd", v);

        // Swap if necessary so u ≤ v
        if u > v {
            std::mem::swap(&mut u, &mut v);
        }

        // Identity 4: gcd(u, v) = gcd(u, v-u) as u ≤ v and u, v are both odd
        v -= u;
        // v is now even

        if v == 0 {
            // Identity 1: gcd(u, 0) = u
            // The shift by k is necessary to add back the 2ᵏ factor that was removed before the loop
            return u << k;
        }

        // Identity 3: gcd(u, 2ʲ v) = gcd(u, v) as u is odd
        v >>= v.trailing_zeros();
    }
}

fn is_gcd_greater_than_1(x: i32, y: i32) -> bool {
    gcd(x.unsigned_abs() as _, y.unsigned_abs() as _) > 1
}

impl Solution {
    pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
        let mut uf = UnionFind::new(nums.len());
        for (outer_idx, &outer_val) in nums.iter().enumerate().skip(1) {
            for (inner_idx, &inner_val) in nums.iter().take(outer_idx).enumerate() {
                if is_gcd_greater_than_1(inner_val, outer_val) {
                    uf.union(inner_idx, outer_idx);
                }
            }
        }

        uf.is_full_rank()
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
    fn case(#[case] nums: Vec<i32>, #[case] expected: bool) {
        let actual = Solution::can_traverse_all_pairs(nums);
        assert_eq!(actual, expected);
    }
}
