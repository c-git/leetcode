//! Solution for https://leetcode.com/problems/min-stack
//! 155. Min Stack

#[derive(Default)]
struct MinStack {
    data: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        Self::default()
    }

    fn push(&mut self, val: i32) {
        let prev_min = self.get_min();
        self.data.push((val, val.min(prev_min)));
    }

    fn pop(&mut self) {
        self.data.pop();
    }

    fn top(&self) -> i32 {
        self.data.last().unwrap().0
    }

    fn get_min(&self) -> i32 {
        self.data.last().map(|(val, min)| *min).unwrap_or(i32::MAX)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn manual_test() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(min_stack.get_min(), -3);
        min_stack.pop();
        assert_eq!(min_stack.top(), 0);
        assert_eq!(min_stack.get_min(), -2);
    }
}
