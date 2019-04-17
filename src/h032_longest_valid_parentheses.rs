/*
 * @lc app=leetcode id=32 lang=rust
 *
 * [32] Longest Valid Parentheses
 *
 * https://leetcode.com/problems/longest-valid-parentheses/description/
 *
 * algorithms
 * Hard (25.19%)
 * Total Accepted:    182.4K
 * Total Submissions: 723.2K
 * Testcase Example:  '"(()"'
 *
 * Given a string containing just the characters '(' and ')', find the length
 * of the longest valid (well-formed) parentheses substring.
 * 
 * Example 1:
 * 
 * 
 * Input: "(()"
 * Output: 2
 * Explanation: The longest valid parentheses substring is "()"
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: ")()())"
 * Output: 4
 * Explanation: The longest valid parentheses substring is "()()"
 * 
 * 
 */
pub struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut max = 0;
        let mut stack = Vec::new();
        for ch in s.into_bytes() {
            if ch == 40 {  // '('
                stack.push(0);
            } else if !stack.is_empty() {  // ')'
                let top = stack.pop().unwrap();
                if top == 0 {
                    if let Some(back) = stack.last_mut() {
                        if *back == 0 {
                            stack.push(2);
                        } else {
                            *back += 2;
                        }
                    } else {
                        stack.push(2);
                    }
                } else {  // top != 0
                    if let Some(_back) = stack.pop() {  // Some(0) | None
                        if let Some(last) = stack.last_mut() {
                            if *last > 0 {
                                *last += top + 2;
                            } else {
                                stack.push(top + 2);
                            }
                        } else {
                            stack.push(top + 2);
                        }
                    } else {
                        max = std::cmp::max(max, top);
                        stack.clear();
                    }
                }
            }
        }
        stack.push(max);
        stack.into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    fn test(s: String, out: i32) {
        assert_eq!(Solution::longest_valid_parentheses(s.clone()), out, "{}", s);
    }
    #[test]
    fn case_0() {
        test("()".into(), 2);
        test("())".into(), 2);
        test("(()".into(), 2);
        test("(()()".into(), 4);
        test("())()".into(), 2);
        test("(())()".into(), 6);
        test("((()))())".into(), 8);
        test("(((((()())()()))()(()))".into(), 22);
    }
}