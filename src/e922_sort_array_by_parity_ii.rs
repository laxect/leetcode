pub struct Solution;

impl Solution {
    pub fn sort_array_by_parity_ii(a: Vec<i32>) -> Vec<i32> {
        let (mut even, mut odd): (Vec<i32>, Vec<i32>) = a.into_iter().partition(|x| x % 2 == 0);
        let mut res = Vec::new();
        while let Some(even_ele) = even.pop() {
            res.push(even_ele);
            res.push(odd.pop().unwrap());
        }
        res
    }
}
