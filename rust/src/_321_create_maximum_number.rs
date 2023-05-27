use std::cmp::Ordering::*;
use std::collections::HashMap;
impl Solution {
    fn merge(mut nums1: &[i32], mut nums2: &[i32], merged: &mut Vec<i32>) {
        while !nums1.is_empty() || !nums2.is_empty() {
            // pop from bigger vector
            if nums1 > nums2 {
                merged.push(nums1[0]);
                nums1 = &nums1[1..];
            } else {
                merged.push(nums2[0]);
                nums2 = &nums2[1..];
            }
        }
    }

    fn get_max_index(nums: &[i32]) -> Option<usize> {
        nums.iter()
            .enumerate()
            .min_by_key(|(idx, &value)| (-value, *idx))
            .map(|(idx, _)| idx)
    }

    pub fn max_number_dp<'a>(
        mut nums1: &[i32],
        mut nums2: &[i32],
        mut k: usize,
        memory: &'a mut HashMap<(usize, usize, usize), Vec<i32>>,
    ) -> &'a Vec<i32> {
        let key = (nums1.len(), nums2.len(), k);
        if memory.contains_key(&key) {
            panic!("Is this used");
            return memory.get(&key).unwrap();
        } else {
            // no of drop allowed to select 1 element
            let mut ans = Vec::with_capacity(k);
            while k > 0 {
                let drop_for_one = nums1.len() + nums2.len() - k;
                if drop_for_one == 0 {
                    Self::merge(nums1, nums2, &mut ans);
                    break;
                } else {
                    let nums1_idx =
                        Self::get_max_index(&nums1[..(drop_for_one + 1).min(nums1.len())]);
                    let nums2_idx =
                        Self::get_max_index(&nums2[..(drop_for_one + 1).min(nums2.len())]);
                    match nums1_idx
                        .and_then(|x| nums1.get(x))
                        .cmp(&nums2_idx.and_then(|x| nums2.get(x)))
                    {
                        Less => {
                            let idx = nums2_idx.unwrap();
                            ans.push(nums2[idx]);
                            nums2 = &nums2[idx + 1..];
                            k -= 1;
                        }
                        Greater => {
                            let idx = nums1_idx.unwrap();
                            ans.push(nums1[idx]);
                            nums1 = &nums1[idx + 1..];
                            k -= 1;
                        }
                        Equal => {
                            let nums2_ = &nums2[nums2_idx.unwrap() + 1..];
                            let nums1_ = &nums1[nums1_idx.unwrap() + 1..];
                            ans.push(nums1[nums1_idx.unwrap()]);

                            let len = ans.len();
                            k -= 1;
                            ans.extend(Self::max_number_dp(nums1_, nums2, k, memory));
                            let nums = Self::max_number_dp(nums1, nums2_, k, memory);
                            if &ans[len..] < nums {
                                ans.truncate(len);
                                ans.extend(nums);
                            }
                            break;
                        }
                    }
                }
            }
            memory.entry(key).or_insert(ans)
        }
    }

    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        // Source: sak96 https://leetcode.com/problems/create-maximum-number/solutions/3538339/fast-solution-using-slice-and-hashmap/
        let mut memory = HashMap::new();
        Self::max_number_dp(&nums1, &nums2, k as usize, &mut memory);
        memory
            .remove(&(nums1.len(), nums2.len(), k as usize))
            .unwrap()
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![3,4,6,5], vec![9,1,2,5,8,3], 5, vec![9,8,6,5,3])]
    #[case(vec![6,7], vec![6,0,4], 5, vec![6,7,6,0,4])]
    #[case(vec![3,9], vec![8,9], 3, vec![9,8,9])]
    #[case(vec![6,7], vec![6,0,4], 4, vec![7,6,0,4])]
    #[case(vec![6,7], vec![6,9,4], 4, vec![9,6,7,4])]
    #[case(vec![9,9], vec![9,9], 1, vec![9])]
    #[case(vec![9,7], vec![9,8], 1, vec![9])]
    fn case(
        #[case] nums1: Vec<i32>,
        #[case] nums2: Vec<i32>,
        #[case] k: i32,
        #[case] expected: Vec<i32>,
    ) {
        let actual = Solution::max_number(nums1, nums2, k);
        assert_eq!(actual, expected);
    }
}
