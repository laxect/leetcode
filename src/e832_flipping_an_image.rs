pub struct Solution {}

impl Solution {
    pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        a.into_iter().map(|mut v| {
            let mut r = v.len() - 1;
            for i in 0..=r/2 {
                let (el, er) = (v[i], v[r]);
                v[r] = el^1;
                v[i] = er^1;
                r -= 1;
            }
            v
        }).collect()
    }
}
