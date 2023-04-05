use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

#[derive(Debug)]
struct Node {
    edge_list: HashSet<i32>,
    is_visited: RefCell<bool>,
}

impl Node {
    fn new() -> Node {
        Self {
            edge_list: HashSet::new(),
            is_visited: RefCell::new(false),
        }
    }
}

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
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
            if !Self::dfs(key, &adjacency_list, num_courses) {
                return false;
            };
        }

        true
    }

    fn dfs(key: i32, adjacency_list: &HashMap<i32, Node>, steps_left: i32) -> bool {
        if steps_left < 0 {
            return false; // Cycle detected taken more steps than there are nodes
        }

        let node = if let Some(node) = adjacency_list.get(&key) {
            node
        } else {
            return true;
        };

        if *node.is_visited.borrow() {
            return true;
        }

        for edge in &node.edge_list {
            if !Self::dfs(*edge, adjacency_list, steps_left - 1) {
                return false;
            };
        }

        *node.is_visited.borrow_mut() = true;

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
