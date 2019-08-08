struct Solution {}

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut l = 0;
        let mut u = 0;
        for ch in moves.as_str().chars() {
            match ch {
                'L' => l += 1,
                'R' => l -= 1,
                'U' => u += 1,
                'D' => u -= 1,
                _ => unreachable!()
            }
        }
        l == 0 && u == 0
    }
}
