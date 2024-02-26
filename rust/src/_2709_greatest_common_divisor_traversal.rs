//! Solution for https://leetcode.com/problems/greatest-common-divisor-traversal
//! 2709. Greatest Common Divisor Traversal

/// Structure to store data for UnionFind
struct UnionFind {
    components: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    /// Creates a new UnionFind of the size passed
    fn new(n: usize) -> Self {
        Self {
            components: (0..n).collect(),
            rank: vec![1; n],
        }
    }

    /// Finds the set that a value belongs to
    fn find(&mut self, x: usize) -> usize {
        if x != self.components[x] {
            self.components[x] = self.find(self.components[x]);
        }
        self.components[x]
    }

    /// Joins the two values passed into one group (into the bigger set)
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

    /// Checks if two values are in the same set
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

/// Move running of sieve_of_eratosthenes to compile time (Mostly just for fun)
const SIEVED_PRIMES: [bool; MAX_VALUE] = sieve_of_eratosthenes();

/// Creates a vector of primes up to (and possibly including) the value passed
///
/// Assumes max is less than the limit given in the question
fn primes(max: usize) -> Vec<usize> {
    SIEVED_PRIMES
        .iter()
        .enumerate()
        .take_while(|(index, _)| index <= &max)
        .filter_map(|(index, &is_prime)| if is_prime { Some(index) } else { None })
        .collect()
}

impl Solution {
    pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
        // Loosely based on https://leetcode.com/problems/greatest-common-divisor-traversal/solutions/4780133/full-detailed-explanation-c-java-js-rust-python-go/
        // and the editorial

        // Special case needed because even if the value in the array is 1 then it is still connected as no traversal is needed
        // To see why it's need consider the case of an array 1's
        if nums.len() == 1 {
            return true;
        }

        // Get the maximum value in the input to avoid allocating more space than needed
        let max = *nums.iter().max().unwrap() as usize;

        // Create a UnionFind to store the connected sets
        let mut union_find = UnionFind::new(max + 1);

        // Get a list of the primes that are up to the max value in the input
        let primes = primes(max);

        // Link numbers in the input to their prime factors (which will link them if they have the same prime factors)
        for &num in nums.iter() {
            if num == 1 {
                // Automatically fail if we find a 1 because it cannot be connected (Special case of a single 1 already handled)
                return false;
            }

            let mut remainder = num as usize; // Start of with the whole number

            // Iteratively remove the prime factors from each number
            while remainder > 1 {
                let factor; // Allocate a space to store the returned value
                (remainder, factor) = remove_prime_factor(remainder, &primes); // Remove a factor
                union_find.union(factor, num as usize); // Link the prime factor removed to the original number from the input
            }
        }

        let first_num = nums[0] as usize; // Test point to see if all other points can reach it

        // Test all other points against the first and return true iff all can reach it
        nums.iter()
            .skip(1)
            .all(|&num| union_find.is_same_set(num as usize, first_num))
    }
}

/// Finds the next factor to remove from `input` and returns the quotient along with the factor removed
///
/// Assumes the `input` is larger than 1 and that primes has the largest prime that is a factor of `input`
fn remove_prime_factor(input: usize, primes: &[usize]) -> (usize, usize) {
    // Check if input is prime
    if primes.binary_search(&input).is_ok() {
        return (1, input);
    }
    let upper_limit = (input as f64).sqrt() as usize;
    let end_index = match primes.binary_search(&upper_limit) {
        Ok(index) => index,
        Err(index) => index.min(primes.len() - 1),
    };
    for i in (0..=end_index).rev() {
        if input % primes[i] == 0 {
            return (input / primes[i], primes[i]);
        }
    }

    unreachable!("should always have a factor")
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
    #[case(vec![1,1], false)]
    #[case(primes(MAX_VALUE).into_iter().map(|x| x as _).collect(), false)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: bool) {
        let actual = Solution::can_traverse_all_pairs(nums);
        assert_eq!(actual, expected);
    }
}
