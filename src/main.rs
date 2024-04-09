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
            self.map.entry(key).and_modify(|v| *v = value);
        } else {
            if self.map.len() == self.capacity {
                if let Some(old_key) = self.order.pop_front() {
                    self.map.remove(&old_key);
                }
            }
            self.map.insert(key, value);
            self.order.push_back(key);
        }

        self.mark_as_recently_used(key)
    }

    fn mark_as_recently_used(&mut self, key: i32) {
        if let Some(position) = self.order.iter().position(|&k| k == key) {
            self.order.remove(position);
        }
        self.order.push_back(key);
    }
}

// 42. 接雨水 单调栈写法
/// Computes the total amount of water that can be trapped between the bars.
///
/// # Arguments
///
/// * `heights`: A slice of integers representing the height of each bar.
///
/// # Returns
///
/// An `i32` representing the total amount of trapped water.
pub fn trap(heights: &[i32]) -> i32 {
    let mut stack = Vec::new();
    let mut water_trapped = 0;

    for (current_index, &current_height) in heights.iter().enumerate() {
        while let Some(&last_index) = stack.last() {
            // If the current height is greater, pop from the stack and calculate trapped water.
            if heights[last_index] < current_height {
                stack.pop(); // We've already checked that stack is not empty.

                if let Some(&second_last_index) = stack.last() {
                    let bounded_height = current_height.min(heights[second_last_index]);
                    water_trapped += (bounded_height - heights[last_index])
                        * (current_index - second_last_index - 1) as i32;
                }
            } else {
                break;
            }
        }
        stack.push(current_index);
    }

    water_trapped
}

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
        assert_eq!(cache.get(1), 1);
        assert_eq!(cache.get(2), 2);

        // Adding a third key-value pair should evict key 1.
        cache.put(3, 3);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);

        // Update key 2, making key 3 least recently used.
        cache.put(2, 22);
        assert_eq!(cache.get(2), 22);

        // Adding a fourth key-value pair should evict key 3.
        cache.put(4, 4);
        assert_eq!(cache.get(3), -1);
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
