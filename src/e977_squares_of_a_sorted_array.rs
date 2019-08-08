
pub struct Solution;

impl Solution {
    pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
        let mut start_point = a.len() as i32;
        for i in 0..a.len() {
            if a[i] >= 0 {
                start_point = i as i32;
                break;
            }
        }
        let mut l = start_point - 1;
        let mut r = start_point;
        let len = a.len() as i32;
        let mut res = Vec::new();
        loop {
            if l >= 0 && r < len {
                let el = -a[l as usize];
                let er = a[r as usize];
                if el > er {
                    res.push(er * er);
                    r += 1;
                } else {
                    res.push(el * el);
                    l -= 1;
                }
            } else if l >= 0 {
                let e = a[l as usize];
                res.push(e * e);
                l -= 1;
            } else if r < len {
                let e = a[r as usize];
                res.push(e * e);
                r += 1;
            } else {
                break;
            }
        }
        res
    }
}
