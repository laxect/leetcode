struct Solution {}

impl Solution {
    pub fn repeated_n_times(a: Vec<i32>) -> i32 {
        let len = a.len();
        if a[0] == a[len-1] {
            return a[0];
        }
        for i in 0..len-1 {
            if a[i] == a[i+1] {
                return a[i];
            } else if i < len-2 && a[i] == a[i+2] {
                return a[i];
            }
        }
        unreachable!()
    }
}
