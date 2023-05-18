use std::collections::HashSet;

impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result: HashSet<i32> = (0..n).collect();

        // Check which vertices have edges leading to them
        for edge in edges {
            let to_vertex = edge[1];
            result.remove(&to_vertex);
        }

        result.into_iter().collect()
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
