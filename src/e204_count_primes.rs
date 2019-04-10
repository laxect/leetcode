/*
 * @lc app=leetcode id=204 lang=rust
 *
 * [204] Count Primes
 *
 * https://leetcode.com/problems/count-primes/description/
 *
 * algorithms
 * Easy (28.61%)
 * Total Accepted:    224.6K
 * Total Submissions: 785K
 * Testcase Example:  '10'
 *
 * Count the number of prime numbers less than a non-negative number, n.
 * 
 * Example:
 * 
 * 
 * Input: 10
 * Output: 4
 * Explanation: There are 4 prime numbers less than 10, they are 2, 3, 5, 7.
 * 
 * 
 */
pub struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n < 2 {
            return 0;
        }
        let mut is_prime = vec![true; n as usize + 1];
        is_prime[0] = false;
        is_prime[1] = false;
        let num = (n as f32).sqrt() as usize;
        for i in 2..=num {
            if is_prime[i] {
                let max = n as usize / i;
                for j in 2..=max {
                    is_prime[i*j] = false;
                }
            }
        }
        is_prime.pop();
        is_prime.into_iter().filter(|x| *x).count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case_10() {
        assert_eq!(Solution::count_primes(10), 4);
    }
}