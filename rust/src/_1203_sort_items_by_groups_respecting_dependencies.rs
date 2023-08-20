//! Solution for https://leetcode.com/problems/sort-items-by-groups-respecting-dependencies
//! 1203. Sort Items by Groups Respecting Dependencies

use std::collections::HashMap;

impl Solution {
    pub fn sort_items(
        n: i32,
        m: i32,
        mut group: Vec<i32>,
        before_items: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        // After reading editorial (code converted from python)
        let n = n as usize;

        // If an item belongs to zero group, assign it a unique group id.
        let mut group_id = m;
        #[allow(clippy::needless_range_loop)]
        for i in 0..n {
            if group[i] == -1 {
                group[i] = group_id;
                group_id += 1;
            }
        }

        // Sort all item regardless of group dependencies.
        let mut item_graph = vec![vec![]; n];
        let mut item_indegree = vec![0; n];

        // Sort all groups regardless of item dependencies.
        let mut group_graph = vec![vec![]; group_id as usize];
        let mut group_indegree = vec![0; group_id as usize];

        for curr in 0..n {
            for &prev in before_items[curr].iter() {
                let prev = prev as usize;

                //  Each (prev -> curr) represents an edge in the item graph.
                item_graph[prev].push(curr);
                item_indegree[curr] += 1;

                // If they belong to different groups, add an edge in the group graph.
                if group[curr] != group[prev] {
                    group_graph[group[prev] as usize].push(group[curr] as usize);
                    group_indegree[group[curr] as usize] += 1;
                }
            }
        }

        /// Topological sort nodes in graph, return [] if a cycle exists.
        fn topological_sort(graph: &[Vec<usize>], indegree: &mut [i32]) -> Vec<usize> {
            let mut visited = vec![];
            let mut stack: Vec<usize> = indegree
                .iter()
                .enumerate()
                .filter_map(|(i, x)| if *x == 0 { Some(i) } else { None })
                .collect();
            #[allow(clippy::manual_while_let_some)]
            while !stack.is_empty() {
                let cur = stack.pop().unwrap();
                visited.push(cur);
                for &neib in graph[cur].iter() {
                    indegree[neib] -= 1;
                    if indegree[neib] == 0 {
                        stack.push(neib)
                    }
                }
            }

            if visited.len() == graph.len() {
                visited
            } else {
                vec![]
            }
        }

        let item_order = topological_sort(&item_graph, &mut item_indegree);
        let group_order = topological_sort(&group_graph, &mut group_indegree);

        if item_order.is_empty() || group_order.is_empty() {
            return vec![];
        }

        // Items are sorted regardless of groups, we need to
        // differentiate them by the groups they belong to.
        let mut ordered_groups: HashMap<i32, Vec<i32>> = HashMap::new();
        for item in item_order {
            ordered_groups
                .entry(group[item])
                .or_default()
                .push(item as i32);
        }

        // Concatenate sorted items in all sorted groups.
        // [group 1, group 2, ... ] -> [(item 1, item 2, ...), (item 1, item 2, ...), ...]
        let mut answer = vec![];
        for group_index in group_order {
            let g = ordered_groups.entry(group_index as i32).or_default();
            answer.append(g);
        }
        answer
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    // Commented out because there are multiple correct solutions and the one from the editorial does not match this
    // #[case(8, 2, vec![-1,-1,1,0,0,1,0,-1], vec![vec![],vec![6],vec![5],vec![6],vec![3,6],vec![],vec![],vec![]], vec![6,3,4,1,5,2,0,7])]
    #[case(8, 2, vec![-1,-1,1,0,0,1,0,-1], vec![vec![],vec![6],vec![5],vec![6],vec![3,6],vec![],vec![],vec![]], vec![7,0,5,2,6,3,4,1])]
    #[case(8, 2, vec![-1,-1,1,0,0,1,0,-1], vec![vec![],vec![6],vec![5],vec![6],vec![3],vec![],vec![4],vec![]], vec![])]
    #[case(5, 5, vec![2,0,-1,3,0], vec![vec![2,1,3],vec![2,4],vec![],vec![],vec![]], vec![2, 4, 1, 3, 0])]
    fn case(
        #[case] n: i32,
        #[case] m: i32,
        #[case] group: Vec<i32>,
        #[case] before_items: Vec<Vec<i32>>,
        #[case] expected: Vec<i32>,
    ) {
        let actual = Solution::sort_items(n, m, group, before_items);
        assert_eq!(actual, expected);
    }
}
