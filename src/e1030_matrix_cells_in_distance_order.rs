pub struct Solution;

impl Solution {
    pub fn all_cells_dist_order(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        res.push(vec![r0, c0]);
        let max_d = vec![
            r0 + c0,
            r - r0 + c0 - 1,
            r - r0 + c - c0 - 2,
            r0 + c - c0 - 1,
        ]
        .into_iter()
        .max()
        .unwrap();
        let mut dis: i32 = 0;
        loop {
            dis += 1;
            if dis > max_d {
                break;
            }
            for xx in -dis..=dis {
                let x = r0 + xx;
                if x < 0 || x >= r {
                    continue;
                }
                let yy = dis - xx.abs();
                // - rng, rng
                let y = c0 + yy;
                if y >= 0 && y < c {
                    res.push(vec![x, y]);
                }
                let y = c0 - yy;
                if yy != 0 && y >= 0 && y < c {
                    res.push(vec![x, y]);
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case_0() {
        assert_eq!(
            Solution::all_cells_dist_order(1, 2, 0, 0),
            vec![vec![0, 0], vec![0, 1]]
        );
    }
}
