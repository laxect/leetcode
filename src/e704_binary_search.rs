/*
 * @lc app=leetcode id=704 lang=rust
 *
 * [704] Binary Search
 *
 * https://leetcode.com/problems/binary-search/description/
 *
 * algorithms
 * Easy (46.51%)
 * Total Accepted:    41.4K
 * Total Submissions: 89K
 * Testcase Example:  '[-1,0,3,5,9,12]\n9'
 *
 * Given a sorted (in ascending order) integer array nums of n elements and a
 * target value, write a function to search target in nums. If target exists,
 * then return its index, otherwise return -1.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [-1,0,3,5,9,12], target = 9
 * Output: 4
 * Explanation: 9 exists in nums and its index is 4
 *
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [-1,0,3,5,9,12], target = 2
 * Output: -1
 * Explanation: 2 does not exist in nums so return -1
 *
 *
 *
 *
 * Note:
 *
 *
 * You may assume that all elements in nums are unique.
 * n will be in the range [1, 10000].
 * The value of each element in nums will be in the range [-9999, 9999].
 *
 *
 */
pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = match nums.len() {
            0 => {
                return -1;
            }
            num => num - 1,
        };
        while l <= r {
            let mid = l + (r-l) / 2;
            if nums[mid] > target {
                r = mid - 1;
            } else if nums[mid] < target {
                l = mid + 1;
            } else {
                return mid as i32;
            }
        };
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case_0() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }
    #[test]
    fn len_0() {
        assert_eq!(Solution::search(vec![], 2), -1);
    }
    #[test]
    fn len_1() {
        assert_eq!(Solution::search(vec![1], 2), -1);
    }
    #[test]
    fn edge_l() {
        assert_eq!(Solution::search(vec![1, 2], 1), 0);
    }
    #[test]
    fn edge_r() {
        assert_eq!(Solution::search(vec![1, 2], 2), 1);
    }
}
