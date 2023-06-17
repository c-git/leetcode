//! Solution for https://leetcode.com/problems/number-of-ways-to-reorder-array-to-get-same-bst
//! 1569. Number of Ways to Reorder Array to Get Same BST

impl Solution {
    // Based on sak96's approach and editorial
    const MOD_BASE: u64 = 1_000_000_007;

    pub fn num_of_ways(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        Self::num_of_ways_helper(&mut nums, &Self::generate_pascal_triangle(n)) as i32 - 1
    }

    /// This function returns the number of ways this tree can be built (including the starting way)
    pub fn num_of_ways_helper(nums: &mut [i32], pascal_triangle: &[Vec<i32>]) -> u64 {
        if nums.len() <= 2 {
            // For a array of size 2 or less there are no changes that can be made because
            // the root if fixed and the root of the child will also be fixed and there will
            // can be no other child
            1
        } else {
            let (root, rest) = nums.split_at_mut(1); // Get rest of array less root (first node)
            let n = rest.len();
            rest.sort_by_key(|i| root[0] < *i); //Use stable sort to split values less than root for left and right child
            let (left, right) = rest.split_at_mut(rest.partition_point(|i| root[0] > *i));
            let mut result = pascal_triangle[n][left.len()] as u64;
            result *= Self::num_of_ways_helper(left, pascal_triangle);
            result %= Self::MOD_BASE;
            result *= Self::num_of_ways_helper(right, pascal_triangle);
            result % Self::MOD_BASE
        }
    }

    fn generate_pascal_triangle(size: usize) -> Vec<Vec<i32>> {
        // This function is mostly here to make testing easy
        let mut result = vec![vec![0; size]; size];

        for (row_index, row) in result.iter_mut().enumerate() {
            row[0] = 1;
            row[row_index] = 1;
        }

        for row in 2..size {
            for col in 1..row {
                result[row][col] =
                    (result[row - 1][col - 1] + result[row - 1][col]) % Self::MOD_BASE as i32;
            }
        }
        result
    }

    fn choose(n: usize, r: usize, pascal_triangle: &[Vec<i32>]) -> i32 {
        pascal_triangle[n][r]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,1,3], 1)]
    #[case(vec![3,4,5,1,2], 5)]
    #[case(vec![1,2,3], 0)]
    #[case(vec![
        34, 45, 67, 21, 51, 58, 65, 42, 20, 6, 13, 18, 14, 15, 33, 7, 55, 57, 32, 48, 53, 62,
        36, 47, 40, 26, 66, 11, 54, 8, 44, 56, 1, 23, 30, 27, 50, 10, 49, 61, 9, 41, 64, 4, 5,
        39, 16, 22, 63, 35, 12, 52, 38, 24, 60, 43, 19, 3, 31, 25, 37, 59, 28, 29, 17, 2, 46,
        ], 236400695)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::num_of_ways(nums);
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(10, 2, 45)]
    #[case(52, 5, 2598960)]
    #[case(100, 5, 75287520)]
    #[case(200, 5, 535650026)]
    #[case(200, 10, 151856252)]
    fn choose(#[case] n: usize, #[case] r: usize, #[case] expected: i32) {
        let table = Solution::generate_pascal_triangle(n + 1);
        assert_eq!(Solution::choose(n, r, &table), expected);
    }
}
