use std::collections::VecDeque;

struct RecentCounter {
    pings: VecDeque<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {

    fn new() -> Self {
        Self {
            pings: VecDeque::new()
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.pings.push_back(t);
        while let Some(f) = self.pings.front() {
            if *f < t-3000 {
                self.pings.pop_front();
            } else {
                break;
            }
        }
        self.pings.len() as i32
    }
}
