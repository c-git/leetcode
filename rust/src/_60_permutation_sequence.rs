//! Solution for https://leetcode.com/problems/permutation-sequence
//! 60. Permutation Sequence

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut seq: Vec<char> = ('1'..=n.to_string().chars().next().unwrap()).collect();
        for _ in 1..k {
            Self::next_permutation(&mut seq);
        }
        seq.into_iter().collect()
    }

    fn next_permutation(nums: &mut [char]) {
        // Taken from 47 and modified
        let n = nums.len();
        if n == 1 {
            return;
        }

        // Find first pair of values where nums[i] < nums[i+1]
        let mut i = n - 2;
        loop {
            if nums[i] < nums[i + 1] {
                break;
            }
            if i > 0 {
                i -= 1;
            } else {
                // Special case when all values need to be reversed
                nums.reverse();
                return;
            }
        }

        // Switch nums[i] with next element larger than it
        // Walk through values to the right of nums[i] and find the min that is still larger
        let right_value_index = nums
            .iter()
            .enumerate()
            .rev()
            .find(|(_index, &x)| x > nums[i])
            .map(|(index, _x)| index)
            .unwrap();
        nums.swap(i, right_value_index);

        // Reverse nums[i+1]
        nums[i + 1..].reverse();
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(3, 3, "213")]
    #[case(4, 9, "2314")]
    #[case(3, 1, "123")]
    fn case(#[case] n: i32, #[case] k: i32, #[case] expected: String) {
        let actual = Solution::get_permutation(n, k);
        assert_eq!(actual, expected);
    }
}
