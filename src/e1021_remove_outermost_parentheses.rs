pub struct Solution;

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut res = String::new();
        let mut stack = 0;
        for ch in s.as_bytes() {
            match ch {
                40 => {
                    if stack != 0 {
                        res.push('(');
                    }
                    stack += 1;
                }
                41 => {
                    if stack > 1 {
                        res.push(')');
                    }
                    stack -= 1;
                }
                _ => unreachable!()
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    fn test(input: &str, output: &str) {
        assert_eq!(Solution::remove_outer_parentheses(input.into()), output.to_string());
    }
    #[test]
    fn case_0() {
        test("(())", "()");
    }
    #[test]
    fn case_1() {
        test("(())(())", "()()");
    }
    #[test]
    fn edge_0() {
        test("()", "");
    }
}
