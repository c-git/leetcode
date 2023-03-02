use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let mut sets: Vec<HashSet<i32>> = vec![];

        for (u, edges) in graph.iter().enumerate() {
            let u = u as i32;

            // Find the set u belongs to or add new set for u and ensure none of it's edges are in that set
            let u_set = Self::get_set(&u, &sets);
            match u_set {
                Some((_, set)) => {
                    // Confirm all edges for u are not in that set
                    for edge in edges {
                        if set.contains(edge) {
                            return false;
                        }
                    }
                }
                None => {
                    let mut new_set = HashSet::new();
                    new_set.insert(u);
                    sets.push(new_set);
                }
            };
            let mut last_index: Option<usize> = None;
            for edge in edges {
                let edge_set = Self::get_set(edge, &sets);
                if let Some((i, _)) = edge_set {
                    if let Some(index) = last_index {
                        if index == i {
                            // Do nothing they are already both added and both the same set
                        } else {
                            let (lower_index, higher_index) =
                                if i < index { (i, index) } else { (index, i) };
                            let other_set = sets.remove(higher_index);
                            sets[lower_index].extend(other_set);
                            last_index = Some(lower_index);
                        }
                    } else {
                        // Nothing to merge with and already added
                        last_index = Some(i);
                    }
                } else if let Some(index) = last_index {
                    // Add to same set as last edge because the new edge doesn't have a set yet
                    sets[index].insert(*edge);
                } else {
                    // No last edge set to use, create a new set and add it to the end of the vector
                    let mut new_set = HashSet::new();
                    new_set.insert(*edge);
                    sets.push(new_set);
                    last_index = Some(sets.len() - 1);
                }
            }
        }
        true
    }

    fn get_set<'a>(u: &i32, sets: &'a [HashSet<i32>]) -> Option<(usize, &'a HashSet<i32>)> {
        sets.iter().enumerate().find(|(_, set)| set.contains(u))
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

#[test]
fn case3() {
    let graph = vec![vec![1], vec![0, 3], vec![3], vec![1, 2]];
    let expected = true;

    let actual = Solution::is_bipartite(graph);
    assert_eq!(actual, expected);
}

#[test]
fn case4() {
    let graph = vec![vec![3], vec![2, 4], vec![1], vec![0, 4], vec![1, 3]];
    let expected = true;

    let actual = Solution::is_bipartite(graph);
    assert_eq!(actual, expected);
}
