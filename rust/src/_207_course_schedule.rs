#[derive(Clone, PartialEq, Eq)]
enum Status {
    NotVisited,
    VisitInProgress,
    VisitCompleted,
}

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        // Source: "Faster Solutions" Modified for readability
        let mut graph = vec![Vec::new(); num_courses as usize];
        let mut visited = vec![Status::NotVisited; num_courses as usize];

        for item in prerequisites {
            graph[item[0] as usize].push(item[1]);
        }

        for i in 0..num_courses {
            if Solution::contains_cycle(&graph, &mut visited, i as usize) {
                return false;
            }
        }

        true
    }

    fn contains_cycle(graph: &Vec<Vec<i32>>, visited: &mut Vec<Status>, course: usize) -> bool {
        if visited[course] == Status::VisitInProgress {
            return true;
        } else if visited[course] == Status::VisitCompleted {
            return false;
        }

        visited[course] = Status::VisitInProgress;
        for &prerequisite in &graph[course] {
            if Solution::contains_cycle(graph, visited, prerequisite as usize) {
                return true;
            }
        }

        visited[course] = Status::VisitCompleted;
        false
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
        let expected = true;
        let actual = Solution::can_finish(num_courses, prerequisites);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let num_courses = 2;
        let prerequisites = vec![vec![1, 0], vec![0, 1]];
        let expected = false;
        let actual = Solution::can_finish(num_courses, prerequisites);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case3() {
        let num_courses = 4;
        let prerequisites = vec![vec![0, 1], vec![2, 1], vec![3, 2]];
        let expected = true;
        let actual = Solution::can_finish(num_courses, prerequisites);
        assert_eq!(actual, expected);
    }
}
