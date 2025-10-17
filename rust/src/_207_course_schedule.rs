//! Solution for https://leetcode.com/problems/course-schedule
//! 207. Course Schedule

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut courses_prerequisites_left: Vec<Vec<usize>> =
            vec![Vec::new(); num_courses as usize];
        let mut dependent_courses: Vec<Vec<usize>> = vec![Vec::new(); num_courses as usize];
        let mut courses_able_to_be_done: Vec<usize> = vec![];

        // Load prerequisite info
        for prerequisite in prerequisites {
            let dependent_course_idx = prerequisite[0] as usize;
            let prerequisite_idx = prerequisite[1] as usize;
            courses_prerequisites_left[dependent_course_idx].push(prerequisite_idx);
            dependent_courses[prerequisite_idx].push(dependent_course_idx);
        }

        // Update the list of those already able to be done
        for (i, course) in courses_prerequisites_left.iter().enumerate() {
            if course.is_empty() {
                courses_able_to_be_done.push(i);
            }
        }

        // Keep doing courses until there are no more courses we can do
        while let Some(course_to_do_idx) = courses_able_to_be_done.pop() {
            for dependent_course_idx in dependent_courses[course_to_do_idx].iter() {
                courses_prerequisites_left[*dependent_course_idx]
                    .retain(|i| i != &course_to_do_idx);
                if courses_prerequisites_left[*dependent_course_idx].is_empty() {
                    courses_able_to_be_done.push(*dependent_course_idx);
                }
            }
        }

        courses_prerequisites_left.iter().all(|x| x.is_empty())
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(2, vec![vec![1,0]], true)]
    #[case(2, vec![vec![1,0],vec![0,1]], false)]
    #[case(4, vec![vec![0, 1], vec![2, 1], vec![3, 2]], true)]
    #[case(5, vec![vec![1,4],vec![2,4],vec![3,1],vec![3,2]], true)]
    fn case(
        #[case] num_courses: i32,
        #[case] prerequisites: Vec<Vec<i32>>,
        #[case] expected: bool,
    ) {
        let actual = Solution::can_finish(num_courses, prerequisites);
        assert_eq!(actual, expected);
    }
}
