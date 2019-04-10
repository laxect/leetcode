/*
 * @lc app=leetcode id=146 lang=rust
 *
 * [146] LRU Cache
 *
 * https://leetcode.com/problems/lru-cache/description/
 *
 * algorithms
 * Hard (24.11%)
 * Total Accepted:    276.2K
 * Total Submissions: 1.1M
 * Testcase Example:  '["LRUCache","put","put","get","put","get","put","get","get","get"]\n[[2],[1,1],[2,2],[1],[3,3],[2],[4,4],[1],[3],[4]]'
 *
 *
 * Design and implement a data structure for Least Recently Used (LRU) cache.
 * It should support the following operations: get and put.
 *
 *
 *
 * get(key) - Get the value (will always be positive) of the key if the key
 * exists in the cache, otherwise return -1.
 * put(key, value) - Set or insert the value if the key is not already present.
 * When the cache reached its capacity, it should invalidate the least recently
 * used item before inserting a new item.
 *
 *
 * Follow up:
 * Could you do both operations in O(1) time complexity?
 *
 * Example:
 *
 * LRUCache cache = new LRUCache( 2 /* capacity */ );
 *
 * cache.put(1, 1);
 * cache.put(2, 2);
 * cache.get(1);       // returns 1
 * cache.put(3, 3);    // evicts key 2
 * cache.get(2);       // returns -1 (not found)
 * cache.put(4, 4);    // evicts key 1
 * cache.get(1);       // returns -1 (not found)
 * cache.get(3);       // returns 3
 * cache.get(4);       // returns 4
 *
 *
 */
use std::collections::{HashMap, LinkedList};

type Pair = (i32, i32);

struct LRUCache {
    len: usize,
    capacity: usize,
    list: LinkedList<Pair>,
    map: HashMap<i32, Pair>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            len: 0,
            capacity: capacity as usize,
            list: LinkedList::new(),
            map: HashMap::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        let res = self.map.get_mut(&key);
        if let Some((ref val, ref mut ver)) = res {
            *ver += 1;
            self.list.push_front((key, *ver));
            return *val;
        } else {
            // not found
            return -1;
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }
        if let Some((ref mut val, ref mut ver)) = self.map.get_mut(&key) {
            // update
            *val = value;
            *ver += 1;
            self.list.push_front((key, *ver));
            return;
        } else {
            self.map.insert(key, (value, 0));
            self.list.push_front((key, 0));
        }
        if self.len < self.capacity {
            self.len += 1;
        } else {
            loop {
                let trash = self.list.pop_back();
                if let Some((ref node_key, ref node_ver)) = trash {
                    let (_map_val, map_ver) = self.map.get(node_key).unwrap();
                    if *map_ver == *node_ver {
                        // the trash
                        self.map.remove(node_key);
                        break;
                    }
                } else {
                    unreachable!();
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn get_test() {
        let mut lru = LRUCache::new(1);
        lru.put(1, 1);
        lru.get(1);
        assert_eq!(lru.map.get(&1), Some(&(1, 1)));
    }
    #[test]
    fn update() {
        let mut lru = LRUCache::new(1);
        lru.put(1, 1);
        lru.put(1, 2);
        assert_eq!(lru.get(1), 2);
    }
    #[test]
    fn lru() {
        let mut lru = LRUCache::new(1);
        lru.put(1, 2);
        lru.put(2, 3);
        assert_eq!(lru.get(1), -1);
    }
    #[test]
    fn newest() {
        let mut lru = LRUCache::new(2);
        lru.put(1, 1);
        lru.put(2, 2);
        assert_eq!(lru.get(1), 1);
        lru.put(3, 3);
        assert_eq!(lru.get(2), -1);
        assert_eq!(lru.map.len(), 2);
    }
    #[test]
    fn case_0() {
        let mut lru_cache = LRUCache::new(2);
        lru_cache.put(1, 1);
        lru_cache.put(2, 2);
        assert_eq!(lru_cache.get(1), 1); // returns 1
        lru_cache.put(3, 3); // evicts key 2
        assert_eq!(lru_cache.get(2), -1); // returns -1 (not found)
        lru_cache.put(4, 4); // evicts key 1
        assert_eq!(lru_cache.get(1), -1); // returns -1 (not found)
        assert_eq!(lru_cache.get(3), 3); // returns 3
        assert_eq!(lru_cache.get(4), 4); // returns 4
    }

    #[test]
    fn case_1() {
        let mut lru_cache = LRUCache::new(1);
        lru_cache.put(2, 1);
        // assert_eq!(lru_cache.get(2), 1);
        lru_cache.put(3, 2);
        // assert_eq!(lru_cache.get(2), -1);
        // assert_eq!(lru_cache.get(3), 2);
    }

    #[test]
    fn case_1000() {
        let mut lru = LRUCache::new(10);
        lru.put(10, 13);
        lru.put(3, 17);
        lru.put(6, 11);
        lru.put(10, 5);
        lru.put(9, 10);
        assert_eq!(lru.get(13), -1);
        lru.put(2, 19);
        assert_eq!(lru.get(2), 19);
        assert_eq!(lru.get(3), 17);
        lru.put(5, 25);
        assert_eq!(lru.get(8), -1);
        lru.put(9, 22);
        lru.put(5, 5);
        lru.put(1, 30);
        assert_eq!(lru.get(11), -1);
        lru.put(9, 12);
        assert_eq!(lru.get(7), -1);
        assert_eq!(lru.get(5), 5);
        assert_eq!(lru.get(8), -1);
        assert_eq!(lru.get(9), 12);
        lru.put(4, 30);
        lru.put(9, 3);
        assert_eq!(lru.get(9), 3);
        assert_eq!(lru.get(10), 5);
        assert_eq!(lru.get(10), 5);
        lru.put(6, 14);
        lru.put(3, 1);
        assert_eq!(lru.get(3), 1);
        lru.put(10, 11);
        assert_eq!(lru.get(8), -1);
        lru.put(2, 14);
        assert_eq!(lru.get(1), 30);
        assert_eq!(lru.get(5), 5);
        assert_eq!(lru.get(4), 30);
        lru.put(11, 4);
        lru.put(12, 24);
        lru.put(5, 18);
        assert_eq!(lru.get(13), -1);
        lru.put(7, 23);
        assert_eq!(lru.get(8), -1);
        assert_eq!(lru.get(12), 24);
        lru.put(3, 27);
        lru.put(2, 12);
        assert_eq!(lru.get(5), 18);
        lru.put(2, 9);
        lru.put(13, 4);
        lru.put(8, 18);
        lru.put(1, 7);
        assert_eq!(lru.get(6), -1);
        lru.put(9, 29);
        lru.put(8, 21);
        assert_eq!(lru.get(5), 18);
        lru.put(6, 30);
        lru.put(1, 12);
        assert_eq!(lru.get(10), -1);
        lru.put(4, 15);
        lru.put(7, 22);
        lru.put(11, 26);
        lru.put(8, 17);
        lru.put(9, 29);
        assert_eq!(lru.get(5), 18);
        lru.put(3, 4);
        lru.put(11, 30);
        assert_eq!(lru.get(12), -1);
        lru.put(4, 29);
        assert_eq!(lru.get(3), 4);
        assert_eq!(lru.get(9), 29);
        assert_eq!(lru.get(6), 30);
        lru.put(3, 4);
        assert_eq!(lru.get(1), 12);
        assert_eq!(lru.get(10), -1);
        lru.put(3, 29);
        lru.put(10, 28);
        lru.put(1, 20);
        lru.put(11, 13);
        assert_eq!(lru.get(3), 29);
        lru.put(3, 12);
        lru.put(3, 8);
        lru.put(10, 9);
        lru.put(3, 26);
        assert_eq!(lru.get(8), 17);
        assert_eq!(lru.get(7), 22);
        assert_eq!(lru.get(5), 18);
        lru.put(13, 17);
        lru.put(2, 27);
        lru.put(11, 15);
        assert_eq!(lru.get(12), -1);
        lru.put(9, 19);
        lru.put(2, 15);
        lru.put(3, 16);
        assert_eq!(lru.get(1), 20);
        lru.put(12, 17);
        lru.put(9, 1);
        lru.put(6, 19);
        assert_eq!(lru.get(4), -1);
        assert_eq!(lru.get(5), 18);
        assert_eq!(lru.get(5), 18);
        lru.put(8, 1);
        lru.put(11, 7);
        lru.put(5, 2);
        lru.put(9, 28);
        assert_eq!(lru.get(1), 20);
        lru.put(2, 2);
        lru.put(7, 4);
        lru.put(4, 22);
        lru.put(7, 24);
        lru.put(9, 26);
        lru.put(13, 28);
        lru.put(11, 26);
    }
}
