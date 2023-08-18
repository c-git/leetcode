//! Solution for https://leetcode.com/problems/permutations
//! 46. Permutations

impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        // Looked up Heaps algorithm
        // https://en.wikipedia.org/wiki/Heap%27s_algorithm
        let mut result = vec![];
        let n = nums.len();
        let mut c = vec![0; n];
        result.push(nums.clone());
        let mut i = 1;
        while i < n {
            if c[i] < i {
                if i % 2 == 0 {
                    nums.swap(0, i);
                } else {
                    nums.swap(c[i], i);
                }
                result.push(nums.clone());
                c[i] += 1;
                i = 1;
            } else {
                c[i] = 0;
                i += 1;
            }
        }
        result
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3], vec![vec![1,2,3],vec![1,3,2],vec![2,1,3],vec![2,3,1],vec![3,1,2],vec![3,2,1]])]
    #[case(vec![0,1], vec![vec![0,1],vec![1,0]])]
    #[case(vec![1], vec![vec![1]])]
    fn case(#[case] nums: Vec<i32>, #[case] mut expected: Vec<Vec<i32>>) {
        let mut actual = Solution::permute(nums);
        actual.sort_unstable();
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }
}
