struct Solution;

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let dis = !x ^ y;
        dis.count_zeros() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case_0() {
        assert_eq!(Solution::hamming_distance(1, 2), 2);
    }
    #[test]
    fn case_1() {
        assert_eq!(Solution::hamming_distance(2, 3), 1);
    }
}
