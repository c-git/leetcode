use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

#[derive(Debug, PartialEq, Eq)]
enum NodeState {
    Unvisited,
    VisitInProgress,
    VisitCompleted,
}

#[derive(Debug)]
struct Node {
    edge_list: HashSet<i32>,
    state: RefCell<NodeState>,
}

impl Node {
    fn new() -> Node {
        Self {
            edge_list: HashSet::new(),
            state: RefCell::new(NodeState::Unvisited),
        }
    }
}

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        // Updated based on https://www.geeksforgeeks.org/detect-cycle-direct-graph-using-colors/
        let mut adjacency_list: HashMap<i32, Node> = HashMap::with_capacity(num_courses as usize);

        for edge in prerequisites {
            let (from, to) = (edge[0], edge[1]);
            if adjacency_list.get(&from).is_none() {
                adjacency_list.insert(from, Node::new());
            }
            let node = adjacency_list
                .get_mut(&from)
                .expect("Should have just been added if not yet present");
            node.edge_list.insert(to);
        }
        dbg!(&adjacency_list);

        let mut keys = vec![];
        for key in adjacency_list.keys() {
            keys.push(*key);
        }

        for key in keys {
            if !Self::dfs(key, &adjacency_list) {
                return false;
            };
        }

        true
    }

    fn dfs(key: i32, adjacency_list: &HashMap<i32, Node>) -> bool {
        let node = if let Some(node) = adjacency_list.get(&key) {
            node
        } else {
            return true;
        };

        match *node.state.borrow() {
            NodeState::Unvisited => (),
            NodeState::VisitInProgress => return false, // Cycle detected
            NodeState::VisitCompleted => return true,
        };

        *node.state.borrow_mut() = NodeState::VisitInProgress;

        for edge in &node.edge_list {
            if !Self::dfs(*edge, adjacency_list) {
                return false;
            };
        }

        *node.state.borrow_mut() = NodeState::VisitCompleted;

        true
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
}
