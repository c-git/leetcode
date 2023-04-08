#[derive(Clone, PartialEq, Eq)]
enum Status {
    NotVisited,
    VisitInProgress,
    VisitCompleted,
}

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        // Use topological sort (adapted solution from 207). Source: https://www.geeksforgeeks.org/topological-sorting/
        let mut graph = vec![Vec::new(); num_courses as usize];
        let mut visited = vec![Status::NotVisited; num_courses as usize];
        let mut result = Vec::with_capacity(num_courses as usize);

        for item in prerequisites {
            graph[item[0] as usize].push(item[1]);
        }

        for i in 0..(num_courses as usize) {
            if visited[i] == Status::VisitCompleted {
                continue;
            }

            Solution::topological_sort(&graph, &mut visited, i, &mut result);

            // If it returns and empty list then no solution is possible because it must include at least 1 node otherwise.
            if result.is_empty() {
                break;
            }
        }

        //result.reverse(); // Not reversed because we want the prerequisites to come first
        result
    }

    /// Produces a reverse topologically sorted set of nodes appended to the result argument or an empty list if there is a cycle
    fn topological_sort(
        graph: &Vec<Vec<i32>>,
        visited: &mut Vec<Status>,
        course: usize,
        result: &mut Vec<i32>,
    ) {
        if visited[course] == Status::VisitInProgress {
            // Cycle exists abort sort
            result.clear();
            return;
        } else if visited[course] == Status::VisitCompleted {
            return; // Already visited do nothing
        }

        visited[course] = Status::VisitInProgress;
        for &prerequisite in &graph[course] {
            Solution::topological_sort(graph, visited, prerequisite as usize, result);
            // If it returns and empty list then no solution is possible because it must include at least 1 node otherwise.
            if result.is_empty() {
                return;
            }
        }

        result.push(course as i32);
        visited[course] = Status::VisitCompleted;
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

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

    #[test]
    fn case1() {
        let num_courses = 2;
        let prerequisites = vec![vec![1, 0]];
        let expected = vec![0, 1];
        let actual = Solution::find_order(num_courses, prerequisites.clone());
        is_valid_order(actual, expected, prerequisites);
    }

    #[test]
    fn case2() {
        let num_courses = 4;
        let prerequisites = vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]];
        let expected = vec![0, 2, 1, 3];
        let actual = Solution::find_order(num_courses, prerequisites.clone());
        is_valid_order(actual, expected, prerequisites);
    }

    #[test]
    fn case3() {
        let num_courses = 1;
        let prerequisites = vec![];
        let expected = vec![0];
        let actual = Solution::find_order(num_courses, prerequisites.clone());
        is_valid_order(actual, expected, prerequisites);
    }
}
