#[allow(clippy::ptr_arg)]
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        // After checking editorial
        let n = nums.len();

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

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3],vec![1,3,2])]
    #[case(vec![3,2,1],vec![1,2,3])]
    #[case(vec![1,1,5],vec![1,5,1])]
    #[case(vec![1,1,1],vec![1,1,1])]
    #[case(vec![1, 2, 3, 4],vec![1, 2, 4, 3])]
    #[case(vec![1, 2, 4, 3],vec![1, 3, 2, 4])]
    #[case(vec![1, 3, 2, 4],vec![1, 3, 4, 2])]
    #[case(vec![1, 3, 4, 2],vec![1, 4, 2, 3])]
    #[case(vec![1, 4, 2, 3],vec![1, 4, 3, 2])]
    #[case(vec![1, 4, 3, 2],vec![2, 1, 3, 4])]
    #[case(vec![2, 1, 3, 4],vec![2, 1, 4, 3])]
    #[case(vec![2, 1, 4, 3],vec![2, 3, 1, 4])]
    #[case(vec![2, 3, 1, 4],vec![2, 3, 4, 1])]
    #[case(vec![2, 3, 4, 1],vec![2, 4, 1, 3])]
    #[case(vec![2, 4, 1, 3],vec![2, 4, 3, 1])]
    #[case(vec![2, 4, 3, 1],vec![3, 1, 2, 4])]
    #[case(vec![3, 1, 2, 4],vec![3, 1, 4, 2])]
    #[case(vec![3, 1, 4, 2],vec![3, 2, 1, 4])]
    #[case(vec![3, 2, 1, 4],vec![3, 2, 4, 1])]
    #[case(vec![3, 2, 4, 1],vec![3, 4, 1, 2])]
    #[case(vec![3, 4, 1, 2],vec![3, 4, 2, 1])]
    #[case(vec![3, 4, 2, 1],vec![4, 1, 2, 3])]
    #[case(vec![4, 1, 2, 3],vec![4, 1, 3, 2])]
    #[case(vec![4, 1, 3, 2],vec![4, 2, 1, 3])]
    #[case(vec![4, 2, 1, 3],vec![4, 2, 3, 1])]
    #[case(vec![4, 2, 3, 1],vec![4, 3, 1, 2])]
    #[case(vec![4, 3, 1, 2],vec![4, 3, 2, 1])]
    #[case(vec![4, 3, 2, 1],vec![1, 2, 3, 4])]

    fn case(#[case] input: Vec<i32>, #[case] expected: Vec<i32>) {
        let mut actual = input.clone();
        Solution::next_permutation(&mut actual);
        assert_eq!(actual, expected, "On input {input:?}");
    }
}
