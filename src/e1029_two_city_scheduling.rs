
struct Solution;

impl Solution {
    pub fn two_city_sched_cost(mut costs: Vec<Vec<i32>>) -> i32 {
        let mut cost = 0;
        costs.sort_unstable_by(|x, y| {
            let a = x[0] - x[1];
            let b = y[0] - y[1];
            if a > b {
                std::cmp::Ordering::Greater
            } else if a == b {
                std::cmp::Ordering::Equal
            } else {
                std::cmp::Ordering::Less
            }
        });
        let len = costs.len();
        let len_a = len/2;
        for i in 0..len {
            if i < len_a {
                cost += costs[i][0];
            } else {
                cost += costs[i][1];
            }
        }
        cost
    }
}
