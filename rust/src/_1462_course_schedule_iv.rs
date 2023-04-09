use std::collections::HashSet;

impl Solution {
    pub fn check_if_prerequisite(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        // Use the in degree based topological sorting algorithm to build a set of courses that each course depends on

        // The to side of an edge is the one that depends on the from side
        let mut graph_out_edges = vec![Vec::new(); num_courses as usize];
        let mut graph_in_edges = vec![HashSet::new(); num_courses as usize];
        let mut zero_degree = vec![];
        let mut depends_on = vec![HashSet::new(); num_courses as usize];
        let mut result = vec![];

        // Build graph
        for item in prerequisites {
            graph_out_edges[item[0] as usize].push(item[1] as usize);
            graph_in_edges[item[1] as usize].insert(item[0] as usize);
        }

        // Collect initial set of courses with no dependencies
        for (i, in_edges) in graph_in_edges.iter().enumerate() {
            if in_edges.is_empty() {
                zero_degree.push(i);
            }
        }

        // Process nodes in zero degree adding nodes as they become zero degree as well
        while !zero_degree.is_empty() {
            let zero_course = zero_degree.pop().unwrap();
            for &dependent_course in graph_out_edges[zero_course].iter() {
                graph_in_edges[dependent_course].remove(&zero_course);
                if graph_in_edges[dependent_course].is_empty() {
                    zero_degree.push(dependent_course);
                }
                depends_on[dependent_course].insert(zero_course);
                let zeros_dependencies = depends_on[zero_course].clone();
                depends_on[dependent_course].extend(zeros_dependencies.into_iter());
            }
        }

        debug_assert_eq!(
            graph_in_edges.iter().map(|edge| edge.len()).sum::<usize>(),
            0,
            "\nExpected all in edges to have been removed"
        );

        for query in queries {
            let (candidate_prerequisite, dependent) = (query[0] as usize, query[1] as usize);
            result.push(depends_on[dependent].contains(&candidate_prerequisite));
        }

        result
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let num_courses = 2;
        let prerequisites = vec![vec![1, 0]];
        let queries = vec![vec![0, 1], vec![1, 0]];
        let expected = vec![false, true];
        let actual = Solution::check_if_prerequisite(num_courses, prerequisites, queries);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let num_courses = 2;
        let prerequisites = vec![];
        let queries = vec![vec![1, 0], vec![0, 1]];
        let expected = vec![false, false];
        let actual = Solution::check_if_prerequisite(num_courses, prerequisites, queries);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case3() {
        let num_courses = 3;
        let prerequisites = vec![vec![1, 2], vec![1, 0], vec![2, 0]];
        let queries = vec![vec![1, 0], vec![1, 2]];
        let expected = vec![true, true];
        let actual = Solution::check_if_prerequisite(num_courses, prerequisites, queries);
        assert_eq!(actual, expected);
    }
}
