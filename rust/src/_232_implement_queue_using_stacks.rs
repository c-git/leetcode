//! Solution for https://leetcode.com/problems/implement-queue-using-stacks
//! 232. Implement Queue using Stacks

#![allow(dead_code)]

struct MyQueue {
    stack1: Vec<i32>,
    stack2: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        Self {
            stack1: vec![],
            stack2: vec![],
        }
    }

    fn push(&mut self, x: i32) {
        self.stack1.push(x);
    }

    fn pop(&mut self) -> i32 {
        if let Some(val) = self.stack2.pop() {
            val
        } else {
            // Fill stack 2 from stack 1 then return the top value (if both empty it panics)
            self.fill_stack2_from_stack1();
            self.stack2.pop().expect("Panics if both are empty")
        }
    }

    fn fill_stack2_from_stack1(&mut self) {
        debug_assert!(
            self.stack2.is_empty(),
            "This should only be called if stack 2 is empty"
        );
        while let Some(val) = self.stack1.pop() {
            self.stack2.push(val);
        }
    }

    fn peek(&mut self) -> i32 {
        if self.stack2.is_empty() {
            self.fill_stack2_from_stack1();
        }

        *self
            .stack2
            .last()
            .expect("Panics if both stacks were empty")
    }

    fn empty(&self) -> bool {
        self.stack1.is_empty() && self.stack2.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        // ["MyQueue","push","push","peek","pop","empty"]
        // [[],[1],[2],[],[],[]]
        let mut obj = MyQueue::new();
        obj.push(1);
        obj.push(2);
        assert_eq!(obj.peek(), 1);
        assert_eq!(obj.pop(), 1);
        assert!(!obj.empty());
        assert_eq!(obj.pop(), 2);
        assert!(obj.empty());
    }
}
