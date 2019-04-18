/*
 * @lc app=leetcode id=58 lang=rust
 *
 * [58] Length of Last Word
 *
 * https://leetcode.com/problems/length-of-last-word/description/
 *
 * algorithms
 * Easy (32.21%)
 * Total Accepted:    258.6K
 * Total Submissions: 802.6K
 * Testcase Example:  '"Hello World"'
 *
 * Given a string s consists of upper/lower-case alphabets and empty space
 * characters ' ', return the length of last word in the string.
 * 
 * If the last word does not exist, return 0.
 * 
 * Note: A word is defined as a character sequence consists of non-space
 * characters only.
 * 
 * Example:
 * 
 * Input: "Hello World"
 * Output: 5
 * 
 * 
 */
struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        if let Some(last) = s.split_whitespace().last() {
            last.len() as i32
        } else {
            0
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case_empty() {
        assert_eq!(Solution::length_of_last_word("  ".into()), 0);
    }
}