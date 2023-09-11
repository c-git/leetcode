//! Solution for https://leetcode.com/problems/group-the-people-given-the-group-size-they-belong-to
//! 1282. Group the People Given the Group Size They Belong To

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut members = vec![None; group_sizes.len() + 1];
        for (i, size) in group_sizes.iter().enumerate() {
            let size = *size as usize;
            if members[size].is_none() {
                members[size] = Some(vec![]);
            }
            members[size].as_mut().unwrap().push(i as i32);
        }
        let mut result = vec![];
        for (size, members) in members.into_iter().enumerate() {
            if let Some(mut members) = members {
                debug_assert!(!members.is_empty());
                debug_assert_eq!(members.len() % size, 0);
                while members.len() > size {
                    let n = members.len();
                    let split_off = members.drain(n - size..).collect();
                    result.push(split_off);
                }
                result.push(members);
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
    #[case(vec![3,3,3,3,3,1,3], vec![vec![5],vec![0,1,2],vec![3,4,6]])]
    #[case(vec![2,1,3,3,3,2], vec![vec![1],vec![0,5],vec![2,3,4]])]
    fn case(#[case] group_sizes: Vec<i32>, #[case] mut expected: Vec<Vec<i32>>) {
        let mut actual = Solution::group_the_people(group_sizes);
        actual.sort_unstable();
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }
}
