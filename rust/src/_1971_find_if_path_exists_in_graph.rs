impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        // Build adjacency graph
        let mut graph: Vec<Vec<usize>> = vec![vec![]; n as usize];
        let mut visited = vec![false; n as usize];
        for edge in edges {
            debug_assert_eq!(edge.len(), 2);
            let (from, to) = (edge[0] as usize, edge[1] as usize);
            graph[from].push(to);
            graph[to].push(from);
        }

        Self::dfs(source as usize, destination as usize, &graph, &mut visited)
    }

    fn dfs(source: usize, destination: usize, graph: &[Vec<usize>], visited: &mut [bool]) -> bool {
        if visited[source] {
            return false;
        }

        visited[source] = true;

        for &other in graph[source].iter() {
            if other == destination || Self::dfs(other, destination, graph, visited) {
                return true;
            }
        }
        false
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 3;
        let edges = [[0, 1], [1, 2], [2, 0]];
        let source = 0;
        let destination = 2;
        let edges = edges.into_iter().map(|x| x.into()).collect();
        let expected = true;
        let actual = Solution::valid_path(n, edges, source, destination);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let n = 6;
        let edges = [[0, 1], [0, 2], [3, 5], [5, 4], [4, 3]];
        let source = 0;
        let destination = 5;
        let edges = edges.into_iter().map(|x| x.into()).collect();
        let expected = false;
        let actual = Solution::valid_path(n, edges, source, destination);
        assert_eq!(actual, expected);
    }
}
