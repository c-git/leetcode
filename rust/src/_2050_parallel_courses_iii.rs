//! Solution for https://leetcode.com/problems/parallel-courses-iii
//! 2050. Parallel Courses III

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

struct Node {
    duration: i32,
    prereqs: HashSet<usize>,
    dependents: Vec<usize>,
}

impl Node {
    fn new(duration: i32) -> Self {
        Self {
            duration,
            prereqs: Default::default(),
            dependents: Default::default(),
        }
    }

    fn add_prereq(&mut self, other_id: usize) {
        self.prereqs.insert(other_id);
    }

    fn add_dependent(&mut self, other_id: usize) {
        self.dependents.push(other_id);
    }

    fn is_blocked(&self) -> bool {
        !self.prereqs.is_empty()
    }
}

impl Solution {
    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut result = 0;

        // Build dependency list
        let mut graph: Vec<Node> = (0..n).map(|id| Node::new(time[id])).collect();
        for relation in relations {
            let (prereq, dependent_course) = (relation[0] as usize - 1, relation[1] as usize - 1);
            graph[dependent_course].add_prereq(prereq);
            graph[prereq].add_dependent(dependent_course);
        }

        // Determine what courses are ready to be taken (no prereqs)
        let mut ongoing_courses = BinaryHeap::new();
        for (id, node) in graph.iter().enumerate() {
            if !node.is_blocked() {
                ongoing_courses.push(Reverse((node.duration, id)));
            }
        }

        // Process completed courses to see which additional courses become unblocked
        while let Some(Reverse((end_time, node_id))) = ongoing_courses.pop() {
            result = result.max(end_time);
            for unblocked_id in graph[node_id].dependents.clone() {
                graph[unblocked_id].prereqs.remove(&node_id);
                if !graph[unblocked_id].is_blocked() {
                    ongoing_courses.push(Reverse((
                        end_time + graph[unblocked_id].duration,
                        unblocked_id,
                    )));
                }
            }
        }

        result
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(3, vec![vec![1,3],vec![2,3]], vec![3,2,5], 8)]
    #[case(5, vec![vec![1,5],vec![2,5],vec![3,5],vec![3,4],vec![4,5]], vec![1,2,3,4,5], 12)]
    fn case(
        #[case] n: i32,
        #[case] relations: Vec<Vec<i32>>,
        #[case] time: Vec<i32>,
        #[case] expected: i32,
    ) {
        let actual = Solution::minimum_time(n, relations, time);
        assert_eq!(actual, expected);
    }
}
