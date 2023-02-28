use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            match seen.get(num) {
                Some(other_index) => return vec![*other_index, i as i32],
                None => {
                    seen.insert(target - num, i as i32);
                }
            }
        }
        unreachable!("By Assumption given in Problem definition")
    }
}

#[test]
fn case1() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let expected = vec![0, 1];

    let actual = Solution::two_sum(nums, target);
    assert_eq!(actual, expected);
}

#[test]
fn case2() {
    let nums = vec![3, 2, 4];
    let target = 6;
    let expected = vec![1, 2];

    let actual = Solution::two_sum(nums, target);
    assert_eq!(actual, expected);
}

#[test]
fn case3() {
    let nums = vec![3, 3];
    let target = 6;
    let expected = vec![0, 1];

    let actual = Solution::two_sum(nums, target);
    assert_eq!(actual, expected);
}
