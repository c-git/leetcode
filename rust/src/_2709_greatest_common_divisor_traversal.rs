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

    fn is_same_set(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
}

const MAX_VALUE: usize = 100_000; //If number is prime increase by 1 so it will be included

/// Returns an array of booleans with the indices set to true being prime
/// Mostly here because I was experimenting with const functions to see what could be offloaded
/// to compile time and shared between runs
const fn sieve_of_eratosthenes() -> [bool; MAX_VALUE] {
    let mut result = [true; MAX_VALUE];
    result[0] = false;
    result[1] = false;
    let mut outer = 2;
    while outer < result.len() {
        if result[outer] {
            let mut inner = outer + outer;
            while inner < result.len() {
                result[inner] = false;
                inner += outer;
            }
        }
        outer += 1;
    }
    result
}

fn primes() -> Vec<usize> {
    sieve_of_eratosthenes()
        .iter()
        .enumerate()
        .filter_map(|(index, &is_prime)| if is_prime { Some(index) } else { None })
        .collect()
}

impl Solution {
    pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
        // Loosely based on https://leetcode.com/problems/greatest-common-divisor-traversal/solutions/4780133/full-detailed-explanation-c-java-js-rust-python-go/

        let max = *nums.iter().max().unwrap() as usize;
        let mut union_find = UnionFind::new(max + 1);
        let primes = primes();

        for &num in nums.iter() {
            let mut remainder = num as usize;
            for &prime in primes.iter() {
                if remainder < prime {
                    break;
                }
                if remainder % prime == 0 {
                    union_find.union(prime, num as usize);
                    remainder /= prime;
                }
            }
        }

        let first_num = nums[0] as usize;
        nums.iter()
            .skip(1)
            .all(|&num| union_find.is_same_set(num as usize, first_num))
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
    #[case(vec![99991], true)]
    #[case(vec![1], true)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: bool) {
        let actual = Solution::can_traverse_all_pairs(nums);
        assert_eq!(actual, expected);
    }
}
