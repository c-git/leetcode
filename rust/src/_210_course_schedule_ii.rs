//! Solution for https://leetcode.com/problems/course-schedule-ii
//! 210. Course Schedule II

use std::collections::HashMap;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![];
        let mut block_count = vec![0u16; num_courses as usize];
        let mut prerequisites_left = prerequisites.into_iter().fold(
            HashMap::<i32, Vec<i32>>::new(),
            |mut map, requirement| {
                let blocked = requirement[0];
                let pre_req = requirement[1];
                map.entry(pre_req).or_default().push(blocked);
                block_count[blocked as usize] += 1;
                map
            },
        );

        let mut unblocked: Vec<i32> = block_count
            .iter()
            .enumerate()
            .filter_map(|(course, &count)| {
                if count == 0 {
                    Some(course as i32)
                } else {
                    None
                }
            })
            .collect();

        while let Some(course) = unblocked.pop() {
            result.push(course);
            if let Some(downstream_blocked) = prerequisites_left.remove(&course) {
                for blocked_course in downstream_blocked {
                    let count = &mut block_count[blocked_course as usize];
                    *count -= 1;
                    if *count == 0 {
                        unblocked.push(blocked_course);
                    }
                }
            }
        }

        if prerequisites_left.is_empty() {
            result
        } else {
            Vec::new()
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::collections::HashMap;

    /// Checks if an actual solution is valid and panics if it is not. If expected is empty then actual must also be empty.
    /// Expected is not used for anything else as the order can be different and still correct
    fn is_valid_order(actual: Vec<i32>, expected: Vec<i32>, prerequisites: Vec<Vec<i32>>) {
        assert_eq!(actual.len(), expected.len());
        if expected.is_empty() {
            return; // Nothing to check no valid solution existed
        }

        // Get positions of each course in the result
        let mut positions = HashMap::with_capacity(actual.len());
        for (i, &val) in actual.iter().enumerate() {
            positions.insert(val, i);
        }

        // Ensure the prerequisite always comes first
        for element in prerequisites {
            let (dependent_course, prerequisite_course) = (element[0], element[1]);
            let (dependent_pos, prerequisite_pos) = (
                positions[&dependent_course],
                positions[&prerequisite_course],
            );
            assert!(prerequisite_pos < dependent_pos,
                "Expected {prerequisite_course} to come before {dependent_course} but found them in positions: {prerequisite_pos} and {dependent_pos} in {actual:?}")
        }
    }
    #[rstest]
    #[case(2, vec![vec![1,0]], vec![0,1])]
    #[case(4, vec![vec![1,0],vec![2,0],vec![3,1],vec![3,2]], vec![0,2,1,3])]
    #[case(1, vec![], vec![0])]
    fn case(
        #[case] num_courses: i32,
        #[case] prerequisites: Vec<Vec<i32>>,
        #[case] expected: Vec<i32>,
    ) {
        let actual = Solution::find_order(num_courses, prerequisites.clone());
        is_valid_order(actual, expected, prerequisites);
    }
}
