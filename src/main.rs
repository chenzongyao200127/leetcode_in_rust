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

// 122. 买卖股票的最佳时机 II
pub fn max_profit_ii(prices: Vec<i32>) -> i32 {
    let mut res = 0;
    prices.windows(2).for_each(|x| {
        if x[1] > x[0] {
            res += x[1] - x[0];
        }
    });
    res
}

// 309. 买卖股票的最佳时机含冷冻期
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let n = prices.len();
    let mut dp = vec![vec![-10000; 2]; n + 1];

    dp[0][0] = 0;

    for (idx, &p) in prices.iter().enumerate() {
        dp[idx + 1][0] = dp[idx][0].max(dp[idx][1] + p);

        if idx >= 1 {
            dp[idx + 1][1] = dp[idx][1].max(dp[idx - 1][0] - p);
        } else {
            dp[idx + 1][1] = dp[idx][1].max(0 - p);
        }
    }

    dp.last().unwrap()[0]
}

// 136. 只出现一次的数字
pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    nums.into_iter().for_each(|x| ans ^= x);
    ans
}

// 169. 多数元素 (摩尔计数)
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut res = nums[0];
    let mut cnt = 1;
    for i in 1..nums.len() {
        if nums[i] == res {
            cnt += 1;
        } else {
            cnt -= 1;
            if cnt == 0 {
                res = nums[i];
                cnt = 1;
            }
        }
    }
    res
}

// 198. 打家劫舍
pub fn rob(nums: Vec<i32>) -> i32 {
    let mut rob = nums[0];
    let mut not_rob = 0;
    for i in 1..nums.len() {
        let new_rob = rob.max(not_rob + nums[i]);
        not_rob = rob.max(not_rob);
        rob = new_rob;
    }

    rob.max(not_rob)
}

// 213. 打家劫舍 II
pub fn rob_ii(nums: Vec<i32>) -> i32 {
    fn rob_linear(nums: &[i32]) -> i32 {
        let mut rob = 0;
        let mut not_rob = 0;
        for &num in nums {
            let new_rob = not_rob + num;
            not_rob = not_rob.max(rob);
            rob = new_rob;
        }
        rob.max(not_rob)
    }

    let n = nums.len();
    if n == 0 {
        0 // No houses to rob
    } else if n == 1 {
        nums[0] // Only one house, so rob it
    } else {
        // Rob houses from 0 to n-2, or from 1 to n-1
        rob_linear(&nums[..n - 1]).max(rob_linear(&nums[1..]))
    }
}

fn main() {
    let res = max_profit(vec![1, 2, 3, 4, 5]);
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
