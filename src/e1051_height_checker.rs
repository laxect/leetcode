struct Solution;
impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut h_sorted = heights.clone();
        h_sorted.sort();
        let mut res = 0;
        for i in 0..heights.len() {
            if heights[i] != h_sorted[i] {
                res += 1;
            }
        }
        res
    }
}
