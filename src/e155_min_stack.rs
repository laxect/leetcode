/*
 * @lc app=leetcode id=155 lang=rust
 *
 * [155] Min Stack
 *
 * https://leetcode.com/problems/min-stack/description/
 *
 * algorithms
 * Easy (36.31%)
 * Total Accepted:    284.7K
 * Total Submissions: 783.4K
 * Testcase Example:  '["MinStack","push","push","push","getMin","pop","top","getMin"]\n[[],[-2],[0],[-3],[],[],[],[]]'
 *
 * 
 * Design a stack that supports push, pop, top, and retrieving the minimum
 * element in constant time.
 * 
 * 
 * push(x) -- Push element x onto stack.
 * 
 * 
 * pop() -- Removes the element on top of the stack.
 * 
 * 
 * top() -- Get the top element.
 * 
 * 
 * getMin() -- Retrieve the minimum element in the stack.
 * 
 * 
 * 
 * 
 * Example:
 * 
 * MinStack minStack = new MinStack();
 * minStack.push(-2);
 * minStack.push(0);
 * minStack.push(-3);
 * minStack.getMin();   --> Returns -3.
 * minStack.pop();
 * minStack.top();      --> Returns 0.
 * minStack.getMin();   --> Returns -2.
 * 
 * 
 */
struct MinStack {
    stack: Vec<i32>,
    min: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min: Vec::new(),
        }
    }
    
    fn push(&mut self, x: i32) {
        self.stack.push(x);
        if let Some(top) = self.min.last() {
            if *top >= x {
                self.min.push(x);
            }
        } else {
            self.min.push(x);
        }
    }
    
    fn pop(&mut self) {
        if let Some(last) = self.stack.pop() {
            if last == *self.min.last().unwrap() {
                self.min.pop();
            }
        }
    }
    
    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }
    
    fn get_min(&self) -> i32 {
        *self.min.last().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case_0() {
        let mut ms = MinStack::new();
        ms.push(-2);
        ms.push(0);
        ms.push(-3);
        assert_eq!(ms.get_min(), -3);
        ms.pop();
        assert_eq!(ms.top(), 0);
        assert_eq!(ms.get_min(), -2);
    }
}