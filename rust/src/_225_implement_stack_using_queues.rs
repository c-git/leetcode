//! Solution for https://leetcode.com/problems/implement-stack-using-queues
//! 225. Implement Stack using Queues

#![allow(dead_code)]

use std::{collections::VecDeque, mem};

struct MyStack {
    emptying_queue: VecDeque<i32>,
    fill_queue: VecDeque<i32>,
    top: i32, // Only valid if queue is not empty
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    fn new() -> Self {
        Self {
            emptying_queue: Default::default(),
            fill_queue: Default::default(),
            top: Default::default(),
        }
    }

    fn push(&mut self, x: i32) {
        self.fill_queue.push_back(x);
        self.top = x;
    }

    fn pop(&mut self) -> i32 {
        let n = self.fill_queue.len();
        debug_assert!(n > 0);
        let mut result = Default::default();
        for i in 0..n {
            let val = self.fill_queue.pop_front().unwrap();

            if i < n - 1 {
                self.top = val;
                self.emptying_queue.push_back(val);
            } else {
                result = val;
            }
        }
        mem::swap(&mut self.emptying_queue, &mut self.fill_queue);
        result
    }

    fn top(&self) -> i32 {
        debug_assert!(!self.empty(), "Top is only valid if queue is not empty");
        self.top
    }

    fn empty(&self) -> bool {
        self.fill_queue.is_empty()
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut my_stack = MyStack::new();
        my_stack.push(1);
        my_stack.push(2);
        assert_eq!(my_stack.top(), 2);
        assert_eq!(my_stack.pop(), 2);
        assert!(!my_stack.empty());
    }

    #[test]
    fn example12() {
        let mut my_stack = MyStack::new();
        my_stack.push(1);
        assert_eq!(my_stack.pop(), 1);
        assert!(my_stack.empty());
    }
}
