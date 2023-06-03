use std::collections::VecDeque;
use std::convert::TryInto;

struct Node {
    inform_time: i32,
    subordinates: Vec<usize>,
}

impl Node {
    fn new(inform_time: i32) -> Self {
        Self {
            inform_time,
            subordinates: Default::default(),
        }
    }

    fn add_subordinate(&mut self, dst: usize) {
        self.subordinates.push(dst);
    }
}

struct OrgChart {
    nodes: Vec<Node>,
}

impl OrgChart {
    fn new(inform_times: Vec<i32>) -> Self {
        let mut nodes = Vec::with_capacity(inform_times.len());
        for inform_time in inform_times {
            nodes.push(Node::new(inform_time));
        }
        Self { nodes }
    }

    fn build(&mut self, managers: Vec<i32>) {
        for (subordinate, manager) in managers.into_iter().enumerate() {
            if let Ok(manager) = manager.try_into() {
                self.add_relationship(manager, subordinate)
            }
        }
    }

    fn add_relationship(&mut self, manager: usize, subordinate: usize) {
        self.nodes[manager].add_subordinate(subordinate);
    }

    fn calculate_max_inform_time(&self, head_id: usize) -> i32 {
        // Changed from DFS (in previous git commit) to BFS to see if there would be an improvement in speed
        let mut result = 0;
        let mut queue = VecDeque::new();
        queue.push_back((head_id, self.nodes[head_id].inform_time));
        while let Some((id, curr_time)) = queue.pop_front() {
            result = result.max(curr_time);
            for &subordinate in self.nodes[id].subordinates.iter() {
                queue.push_back((subordinate, curr_time + self.nodes[subordinate].inform_time));
            }
        }
        result
    }
}

impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        debug_assert_eq!(n as usize, inform_time.len());
        debug_assert_eq!(n as usize, manager.len());
        let mut org_chart = OrgChart::new(inform_time);
        org_chart.build(manager);
        org_chart.calculate_max_inform_time(head_id as usize)
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1, 0, vec![-1], vec![0], 0)]
    #[case(6, 2, vec![2,2,-1,2,2,2], vec![0,0,1,0,0,0], 1)]
    fn case(
        #[case] n: i32,
        #[case] head_id: i32,
        #[case] manager: Vec<i32>,
        #[case] inform_time: Vec<i32>,
        #[case] expected: i32,
    ) {
        let actual = Solution::num_of_minutes(n, head_id, manager, inform_time);
        assert_eq!(actual, expected);
    }
}
