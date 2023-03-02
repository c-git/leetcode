use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let mut a = HashSet::<i32>::new();
        let mut b = HashSet::<i32>::new();
        for (u, edges) in graph.iter().enumerate() {
            let u = u as i32;
            let mut a_local: &mut HashSet<i32>;
            let mut b_local: &mut HashSet<i32>;
            if a.contains(&u) || !b.contains(&u) {
                a_local = &mut a;
                b_local = &mut b;
            } else {
                a_local = &mut b;
                b_local = &mut a;
            }
            a_local.insert(u);
            for edge in edges {
                if a_local.contains(edge) {
                    return false; // Both in same set
                }
                b_local.insert(*edge);
            }
        }
        true
    }
}

#[test]
fn case1() {
    let graph = vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]];
    let expected = false;

    let actual = Solution::is_bipartite(graph);
    assert_eq!(actual, expected);
}

#[test]
fn case2() {
    let graph = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];
    let expected = true;

    let actual = Solution::is_bipartite(graph);
    assert_eq!(actual, expected);
}
