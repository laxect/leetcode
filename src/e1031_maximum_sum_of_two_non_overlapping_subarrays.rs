
pub struct Solution;

impl Solution {
    pub fn solve(a: &[i32], l: i32, m: i32) -> i32 {
        // pre
        let mut s = 0;
        let mut sum = Vec::new();
        sum.push(0);
        for num in a.iter() {
            s += *num;
            sum.push(s);
        }
        // sum(i..j) = sum[j] - sum[i]
        let mut res = 0;
        let m_end = a.len() - m as usize;
        let l_end = m_end - l as usize;
        for l_start in 0..=l_end {
            for m_start in (l_start + l as usize)..=m_end {
                let l_sum = sum[l_start + l as usize] - sum[l_start];
                let m_sum = sum[m_start + m as usize] - sum[m_start];
                let sum = l_sum + m_sum;
                res = std::cmp::max(sum, res);
            }
        }
        res
    }

    pub fn max_sum_two_no_overlap(a: Vec<i32>, l: i32, m: i32) -> i32 {
        std::cmp::max(Solution::solve(&a, l, m), Solution::solve(&a, m, l))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case_0() {
        assert_eq!(Solution::max_sum_two_no_overlap(vec![1, 0, 2, 4], 2, 1), 7);
    }
}
