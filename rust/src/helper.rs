use std::{
    cell::RefCell,
    collections::VecDeque,
    fmt::{Debug, Formatter},
    rc::Rc,
};

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[derive(PartialEq, Eq)]
pub struct ListHead {
    head: Option<Box<ListNode>>,
}

impl Debug for ListHead {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let head: Vec<i32> = self.into();
        head.fmt(f)
    }
}

impl ListHead {
    fn new(head: Option<Box<ListNode>>) -> Self {
        ListHead { head }
    }
}

impl From<ListHead> for Option<Box<ListNode>> {
    fn from(value: ListHead) -> Self {
        value.head
    }
}

impl From<Option<Box<ListNode>>> for ListHead {
    fn from(head: Option<Box<ListNode>>) -> Self {
        Self { head }
    }
}

impl From<Vec<i32>> for ListHead {
    fn from(values: Vec<i32>) -> Self {
        // Reverse version before looking at
        // https://github.com/zwhitchcox/leetcode/blob/master/src/0002_add_two_numbers.rs
        // to see how it could be done going forward instead of backward
        //
        // let mut last: Option<Box<ListNode>> = None;
        // for &n in values.iter().rev() {
        //     let mut temp = ListNode::new(n);
        //     temp.next = last;
        //     last = Some(Box::new(temp));
        // }
        // ListHead::new(last)

        let mut result = Self { head: None };
        let mut curr = &mut result.head;
        for &num in &values {
            let node = ListNode::new(num);
            *curr = Some(Box::new(node));
            curr = &mut curr.as_mut().unwrap().next;
        }
        result
    }
}

impl From<&ListHead> for Vec<i32> {
    fn from(list_head: &ListHead) -> Self {
        let mut result = vec![];
        let mut curr = &list_head.head;
        while let Some(node) = &curr {
            result.push(node.val);
            curr = &node.next;
        }
        result
    }
}

#[derive(PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
    pub fn new_maybe(val: Option<i32>) -> Option<Self> {
        val.map(|val| TreeNode {
            val,
            left: None,
            right: None,
        })
    }
    fn wrapped_node(val: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new(val))))
    }
    fn wrapped_node_maybe(val: Option<i32>) -> Option<Rc<RefCell<Self>>> {
        val.map(|x| Rc::new(RefCell::new(Self::new(x))))
    }
}

#[derive(PartialEq, Eq)]
pub struct TreeRoot {
    pub root: Option<Rc<RefCell<TreeNode>>>,
}

impl Debug for TreeRoot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut vec: Vec<Option<i32>> = self.into();

        // Remove trailing None's
        while !vec.is_empty() && vec[vec.len() - 1].is_none() {
            let _ = vec.remove(vec.len() - 1);
        }

        let vec: Vec<String> = vec
            .iter()
            .map(|x| {
                if let Some(x) = x {
                    format!("{x}")
                } else {
                    "None".to_string()
                }
            })
            .collect();
        write!(f, "{vec:?}")
    }
}

impl From<&TreeRoot> for Vec<Option<i32>> {
    fn from(value: &TreeRoot) -> Self {
        let mut result = vec![];
        let mut deque = VecDeque::new();
        if value.root.is_some() {
            deque.push_back(value.root.clone());
        }
        while !deque.is_empty() {
            let node = deque.pop_front().unwrap();
            match &node {
                Some(node) => {
                    let node = node.as_ref().borrow();
                    result.push(Some(node.val));
                    deque.push_back(node.left.clone());
                    deque.push_back(node.right.clone());
                }
                None => {
                    result.push(None);
                }
            }
        }
        result
    }
}

impl From<Option<Rc<RefCell<TreeNode>>>> for TreeRoot {
    fn from(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self { root }
    }
}

impl From<&str> for TreeRoot {
    /// Expects the "[]" around the values, separated by comma "," and only integers and "null"
    /// (which is the format you'll get on LeetCode)
    ///
    /// # Panics
    ///
    /// This function panics if it doesn't match the expected format
    fn from(value: &str) -> Self {
        let mut result: Vec<Option<i32>>;
        // Check and remove "[]"
        assert!(value.len() >= 2);
        assert_eq!('[', value.chars().next().unwrap());
        assert_eq!(']', value.chars().nth(value.len() - 1).unwrap());
        if value.len() == 2 {
            // Empty array return empty tree
            return Self { root: None };
        }
        let value = &value[1..value.len() - 1];

        // Separate by comma
        let values: Vec<&str> = value.split(',').map(|v| v.trim()).collect();

        // Convert into values
        result = vec![];
        for value in values {
            result.push(if value == "null" {
                None
            } else {
                Some(value.parse().unwrap())
            })
        }

        result.into()
    }
}

impl Debug for TreeNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let left = if let Some(left) = &self.left {
            format!("{:?}", left.as_ref().borrow())
        } else {
            "None".to_string()
        };
        let right = if let Some(right) = &self.right {
            format!("{:?}", right.as_ref().borrow())
        } else {
            "None".to_string()
        };
        write!(f, "{{val:{} left:{} right:{}}}", self.val, left, right)
    }
}

impl From<Vec<i32>> for TreeRoot {
    fn from(value: Vec<i32>) -> Self {
        value
            .iter()
            .map(|x| Some(*x))
            .collect::<Vec<Option<i32>>>()
            .into()
    }
}

impl From<Vec<Option<i32>>> for TreeRoot {
    /// Converts the incoming vec into a tree
    /// Would be more efficient if the incoming vec were converted to a dequeue as we keep popping from the front
    fn from(mut list: Vec<Option<i32>>) -> Self {
        if list.is_empty() {
            return TreeRoot { root: None };
        }

        let first_element = list.remove(0); // Get first element
        if first_element.is_none() {
            return TreeRoot { root: None };
        }

        let root = Self {
            root: TreeNode::wrapped_node(first_element.expect("Checked for None above")),
        };

        let mut deque = VecDeque::<Rc<RefCell<TreeNode>>>::new();
        deque.push_back(root.root.as_ref().unwrap().clone());
        while !deque.is_empty() && !list.is_empty() {
            let node = deque.pop_front().expect("Just check for non empty");

            // Get left child
            if list.is_empty() {
                break;
            }
            let child = list.remove(0);
            if let Some(child) = child {
                let child = TreeNode::wrapped_node(child);
                node.borrow_mut().left = child.clone();
                deque.push_back(child.unwrap());
            }

            // Get right child
            if list.is_empty() {
                break;
            }
            let child = list.remove(0);
            if let Some(child) = child {
                let child = TreeNode::wrapped_node(child);
                node.borrow_mut().right = child.clone();
                deque.push_back(child.unwrap());
            }
        }
        root
    }
}

impl From<TreeRoot> for Option<Rc<RefCell<TreeNode>>> {
    fn from(value: TreeRoot) -> Self {
        value.root
    }
}
