impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let n = nums.len();
        let mut next_unchecked = 0;
        let mut discard_count = 0;
        while next_unchecked < n - discard_count {
            if nums[next_unchecked] == val {
                // Discard by swapping to back
                nums.swap(next_unchecked, n - 1 - discard_count);
                discard_count += 1;
            } else {
                next_unchecked += 1;
            }
        }
        (n - discard_count) as i32
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![3,2,2,3], 3, vec![2,2])]
    #[case(vec![0,1,2,2,3,0,4,2], 2, vec![0,1,4,0,3])]
    #[case(vec![0,1,2,2,3,0,4,2], 8, vec![0,1,2,2,3,0,4,2])]
    fn case(#[case] mut nums: Vec<i32>, #[case] val: i32, #[case] mut expected: Vec<i32>) {
        let k = Solution::remove_element(&mut nums, val) as usize;
        let actual = &mut nums[0..k];
        actual.sort_unstable();
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }
}
