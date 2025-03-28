//! Solution for https://leetcode.com/problems/first-missing-positive
//! 41. First Missing Positive

// [0,1,2]
impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        for mut index in 0..nums.len() {
            let mut num = nums[index];
            loop {
                if num < 1 || num > n {
                    // We aren't interested in values outside of 1..n
                    break;
                }

                // Not a valid usize and we are just going to skip negative values
                let Ok(index_candidate) = usize::try_from(num) else {
                    unreachable!("we already checked for negative values")
                };
                let index_candidate = index_candidate - 1; // So 1 -> 0, 2 -> 1 and so on

                if index == index_candidate {
                    // Already in the right place
                    break;
                }

                num = nums[index_candidate]; // Store next value in chain before we overwrite
                nums[index_candidate] = index_candidate as i32 + 1; // store index in it's place so we know it's done
                index = index_candidate; // Save index for next loop
            }
        }
        nums.into_iter()
            .enumerate()
            .find_map(|(i, num)| {
                if num > 0 && num == i as i32 + 1 {
                    None
                } else {
                    Some(i as i32 + 1)
                }
            })
            .unwrap_or(n + 1)
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
    #[case(vec![2,1], 3)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::first_missing_positive(nums);
        assert_eq!(actual, expected);
    }
}
