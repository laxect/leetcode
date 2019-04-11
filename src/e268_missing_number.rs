/*
 * @lc app=leetcode id=268 lang=rust
 *
 * [268] Missing Number
 *
 * https://leetcode.com/problems/missing-number/description/
 *
 * algorithms
 * Easy (47.92%)
 * Total Accepted:    260.8K
 * Total Submissions: 544.2K
 * Testcase Example:  '[3,0,1]'
 *
 * Given an array containing n distinct numbers taken from 0, 1, 2, ..., n,
 * find the one that is missing from the array.
 *
 * Example 1:
 *
 *
 * Input: [3,0,1]
 * Output: 2
 *
 *
 * Example 2:
 *
 *
 * Input: [9,6,4,2,3,5,7,0,1]
 * Output: 8
 *
 *
 * Note:
 * Your algorithm should run in linear runtime complexity. Could you implement
 * it using only constant extra space complexity?
 */
struct Solution;

impl Solution {
    pub fn missing_number(mut nums: Vec<i32>) -> i32 {
        let mut pos_n = 0;
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        for i in 0..n {
            if i == nums[i] as usize || i == n {
                continue;
            }
            loop {
                let nxt = nums[i] as usize;
                if nxt != n {
                    nums.swap(nxt, i);
                } else {
                    pos_n = i;
                    break;
                }
                if nums[i] as usize == i {
                    break;
                }
            }
        }
        match pos_n {
            0 => match nums[0] {
                0 => n as i32,
                _ => 0,
            }
            _ => pos_n as i32
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case_2() {
        assert_eq!(Solution::missing_number(vec![0, 2]), 1);
    }
    #[test]
    fn case_3() {
        assert_eq!(Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }
    #[test]
    fn case_1() {
        assert_eq!(Solution::missing_number(vec![0]), 1);
        assert_eq!(Solution::missing_number(vec![1]), 0);
    }
    #[test]
    fn case_0() {
        assert_eq!(Solution::missing_number(vec![]), 0);
    }
}