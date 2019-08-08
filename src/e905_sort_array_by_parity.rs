pub struct Solution {}

impl Solution {
    pub fn sort_array_by_parity(mut a: Vec<i32>) -> Vec<i32> {
        let mut r = a.len() - 1;
        let mut i = 0;
        loop {
            if i >= r {
                break;
            }
            if a[i] % 2 != 0 {
                a.swap(i, r);
                r -= 1;
            } else {
                i += 1;
            }
        }
        a
    }
}
