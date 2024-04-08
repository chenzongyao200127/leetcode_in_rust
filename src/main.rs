// 1. 实现 `LRUCache` 类：

//     `LRUCache(int capacity)` 以 **正整数** 作为容量 `capacity` 初始化 LRU 缓存

//     `int get(int key)` 如果关键字 `key` 存在于缓存中，则返回关键字的值，否则返回 `1` 。

//     `void put(int key, int value)` 如果关键字 `key` 已经存在，则变更其数据值 `value` ；
// 如果不存在，则向缓存中插入该组 `key-value` 。如果插入操作导致关键字数量超过 `capacity` ，则应该 **逐出** 最久未使用的关键字。

//     函数 `get` 和 `put` 必须以 `O(1)` 的平均时间复杂度运行。

// **示例：**
// 输入
// ["LRUCache", "put", "put", "get", "put", "get", "put", "get", "get", "get"]
// [[2], [1, 1], [2, 2], [1], [3, 3], [2], [4, 4], [1], [3], [4]]
// 输出
// [null, null, null, 1, null, -1, null, -1, 3, 4]

// 解释
// LRUCache lRUCache = new LRUCache(2);
// lRUCache.put(1, 1); // 缓存是 {1=1}
// lRUCache.put(2, 2); // 缓存是 {1=1, 2=2}
// lRUCache.get(1);    // 返回 1
// lRUCache.put(3, 3); // 该操作会使得关键字 2 作废，缓存是 {1=1, 3=3}
// lRUCache.get(2);    // 返回 -1 (未找到)
// lRUCache.put(4, 4); // 该操作会使得关键字 1 作废，缓存是 {4=4, 3=3}
// lRUCache.get(1);    // 返回 -1 (未找到)
// lRUCache.get(3);    // 返回 3
// lRUCache.get(4);    // 返回 4

use std::collections::{HashMap, VecDeque};

struct LRUCache {
    capacity: usize,
    map: HashMap<i32, i32>,
    order: VecDeque<i32>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity: capacity as usize,
            map: HashMap::new(),
            order: VecDeque::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(&value) = self.map.get(&key) {
            self.mark_as_recently_used(key);
            value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.map.contains_key(&key) {
            // Update the value for an existing key.
            self.map.insert(key, value);
            self.mark_as_recently_used(key);
        } else {
            // Ensure the cache does not exceed its capacity.
            if self.map.len() == self.capacity {
                if let Some(old_key) = self.order.pop_front() {
                    self.map.remove(&old_key);
                }
            }
            self.map.insert(key, value);
            self.order.push_back(key);
        }
    }

    fn mark_as_recently_used(&mut self, key: i32) {
        // Remove the key from its current position in the order queue.
        if let Some(position) = self.order.iter().position(|&k| k == key) {
            self.order.remove(position);
        }
        // Insert the key at the back of the order queue.
        self.order.push_back(key);
    }
}

// /**
//  * Your LRUCache object will be instantiated and called as such:
//  * let obj = LRUCache::new(capacity);
//  * let ret_1: i32 = obj.get(key);
//  * obj.put(key, value);
//  */
fn main() {
    let capacity = 2;
    let mut obj = LRUCache::new(capacity);

    let key = 1;
    let ret_1: i32 = obj.get(key);
    let value = 2;
    obj.put(key, value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_cache() {
        let mut cache = LRUCache::new(2);

        // -1 because cache is empty
        assert_eq!(cache.get(1), -1);

        // Add a key-value pair.
        cache.put(1, 1);
        assert_eq!(cache.get(1), 1);

        // Add another key-value pair and check order.
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1); // Key 1 is now least recently used.
        assert_eq!(cache.get(2), 2);

        // Adding a third key-value pair should evict key 1.
        cache.put(3, 3);
        assert_eq!(cache.get(1), -1); // Key 1 should be evicted.
        assert_eq!(cache.get(3), 3);

        // Update key 2, making key 3 least recently used.
        cache.put(2, 22);
        assert_eq!(cache.get(2), 22);

        // Adding a fourth key-value pair should evict key 3.
        cache.put(4, 4);
        assert_eq!(cache.get(3), -1); // Key 3 should be evicted.
        assert_eq!(cache.get(4), 4);
    }
}

// 1. 给你一个整数数组 `prices` 和一个整数 `k` ，其中 `prices[i]` 是某支给定的股票在第 `i` **天的价格。
//     设计一个算法来计算你所能获取的最大利润。你最多可以完成 `k` 笔交易。也就是说，你最多可以买 `k` 次，卖 `k` 次。
//     **注意：**你不能同时参与多笔交易（你必须在再次购买前出售掉之前的股票）。

// **示例 1：**
// 输入：k = 2, prices = [2,4,1]
// 输出：2
// 解释：在第 1 天 (股票价格 = 2) 的时候买入，在第 2 天 (股票价格 = 4) 的时候卖出，这笔交易所能获得利润 = 4-2 = 2 。

// **示例 2：**
// 输入：k = 2, prices = [3,2,6,5,0,3]
// 输出：7
// 解释：在第 2 天 (股票价格 = 2) 的时候买入，在第 3 天 (股票价格 = 6) 的时候卖出, 这笔交易所能获得利润 = 6-2 = 4 。
//      随后，在第 5 天 (股票价格 = 0) 的时候买入，在第 6 天 (股票价格 = 3) 的时候卖出, 这笔交易所能获得利润 = 3-0 = 3 。
