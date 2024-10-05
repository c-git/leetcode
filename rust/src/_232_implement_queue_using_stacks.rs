//! Solution for https://leetcode.com/problems/implement-queue-using-stacks
//! 232. Implement Queue using Stacks

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
            stack1: Default::default(),
            stack2: Default::default(),
        }
    }

    fn push(&mut self, x: i32) {
        self.stack1.push(x);
    }

    fn pop(&mut self) -> i32 {
        if self.stack2.is_empty() {
            // Only transfers items if stack2 is empty to preserve order
            while let Some(item) = self.stack1.pop() {
                self.stack2.push(item);
            }
        }
        self.stack2
            .pop()
            .expect("should have an item by problem constraints")
    }

    fn peek(&self) -> i32 {
        if let Some(last) = self.stack2.last() {
            last
        } else {
            self.stack1
                .first()
                .expect("should have an item by problem constraints")
        }
        .to_owned()
    }

    fn empty(&self) -> bool {
        self.stack1.is_empty() && self.stack2.is_empty()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */

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
