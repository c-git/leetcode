//! Solution for https://leetcode.com/problems/first-missing-positive
//! 41. First Missing Positive

// [0,1,2]
impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return if nums[0] == 1 { 2 } else { 1 };
        }
        for mut index in 0..nums.len() {
            let mut num = nums[index];
            loop {
                let Ok(index_candidate) = usize::try_from(num) else {
                    // Not a valid usize and we are just going to skip negative values
                    break;
                };

                if index_candidate >= nums.len() {
                    // Not a valid index we can skip because then a number from 1..nums.len() will
                    // be missing and that will be our answer
                    break;
                }

                if index == index_candidate {
                    // Already in the right place
                    break;
                }

                num = nums[index_candidate]; // Store next value in chain before we overwrite
                nums[index_candidate] = index_candidate as _; // store index in it's place so we know it's done
                index = index_candidate; // Save index for next loop
            }
        }
        let n = nums.len();
        nums.into_iter()
            .enumerate()
            .skip(1)
            .find_map(|(i, num)| {
                if num == i as i32 {
                    None
                } else {
                    Some(i as i32)
                }
            })
            .unwrap_or(n as i32)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,0], 3)]
    #[case(vec![3,4,-1,1], 2)]
    #[case(vec![7,8,9,11,12], 1)]
    #[case(vec![1], 2)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::first_missing_positive(nums);
        assert_eq!(actual, expected);
    }
}
