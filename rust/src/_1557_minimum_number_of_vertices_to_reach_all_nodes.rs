use std::collections::{HashSet, VecDeque};

impl Solution {
    /// Intuition - Any vertex: with a parent is not a root. Doesn't matter how many nodes point to it.
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut result = HashSet::new();
        let mut is_visited = vec![false; n];

        // Create graph
        let mut graph = vec![vec![]; n];
        for edge in edges {
            graph[edge[0] as usize].push(edge[1] as usize);
        }

        // Do BFS from each node to see who can be reached
        let mut queue = VecDeque::new();
        for i in 0..n {
            if is_visited[i] {
                continue;
            }
            result.insert(i);
            queue.push_back(i);
            is_visited[i] = true;
            while let Some(node) = queue.pop_front() {
                for &neighbour in graph[node].iter() {
                    if !is_visited[neighbour] {
                        is_visited[neighbour] = true;
                        queue.push_back(neighbour);
                    } else {
                        result.remove(&neighbour);
                    }
                }
            }
        }

        result.into_iter().map(|x| x as i32).collect()
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    fn validate(mut actual: Vec<i32>, mut expected: Vec<i32>) {
        actual.sort_unstable();
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }

    #[test]
    fn case1() {
        let n = 6;
        let edges = [[0, 1], [0, 2], [2, 5], [3, 4], [4, 2]];
        let edges = edges.into_iter().map(|x| x.into()).collect();
        let expected = [0, 3];
        let actual = Solution::find_smallest_set_of_vertices(n, edges);
        validate(actual, expected.into());
    }

    #[test]
    fn case2() {
        let n = 5;
        let edges = [[0, 1], [2, 1], [3, 1], [1, 4], [2, 4]];
        let edges = edges.into_iter().map(|x| x.into()).collect();
        let expected = [0, 2, 3];
        let actual = Solution::find_smallest_set_of_vertices(n, edges);
        validate(actual, expected.into());
    }

    #[test]
    fn case3() {
        let n = 6;
        let edges = [[0, 1], [0, 2], [2, 5], [3, 4], [4, 2], [0, 3]];
        let edges = edges.into_iter().map(|x| x.into()).collect();
        let expected = [0];
        let actual = Solution::find_smallest_set_of_vertices(n, edges);
        validate(actual, expected.into());
    }

    #[test]
    fn case4() {
        let n = 6;
        let edges = [[1, 5], [0, 1]];
        let edges = edges.into_iter().map(|x| x.into()).collect();
        let expected = [0, 2, 3, 4];
        let actual = Solution::find_smallest_set_of_vertices(n, edges);
        validate(actual, expected.into());
    }
}
