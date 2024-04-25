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

// 547. 省份数量
pub struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
        }
    }

    pub fn find_set(&self, x: usize) -> i32 {
        if self.parent[x] != x {
            return self.find_set(self.parent[x]);
        } else {
            return x as i32;
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let root1 = self.find_set(x);
        let root2 = self.find_set(y);

        if root1 == root2 {
            return false;
        } else {
            // union
            self.parent[root2 as usize] = root1 as usize;
            return true;
        }
    }
}

pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    let cities = is_connected.len();
    let mut uf = UnionFind::new(cities);

    for i in 0..cities {
        for j in 0..cities {
            if is_connected[i][j] == 1 {
                uf.unite(i, j);
            }
        }
    }

    let mut ans = 0;
    (0..cities).into_iter().for_each(|x| {
        if uf.find_set(x) == x as i32 {
            ans += 1;
        }
    });
    ans
}

// 684. 冗余连接
pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = edges.len();
    let mut nf = UnionFind::new(n + 1);

    let mut res = vec![];

    for edge in edges {
        let x = edge[0];
        let y = edge[1];
        if nf.find_set(x as usize) == nf.find_set(y as usize) {
            res = edge;
        } else {
            nf.unite(x as usize, y as usize);
        }
    }

    res
}

// 2560. 打家劫舍 IV
// https://leetcode.cn/problems/house-robber-iv/description/

/// 为什么 看到「最大化最小值」或者「最小化最大值」就要想到二分答案？
pub fn min_capability(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();
    let n = nums.len();

    // 贪心
    #[inline]
    fn solve(nums: &Vec<i32>, ms: i32, k: i32) -> bool {
        let mut res = 0;
        for i in (0..nums.len()).step_by(2) {
            if nums[i] <= ms {
                res += 1;
            }
        }
        res >= k
    }

    let mut l = 0;
    let mut r = n - 1;
    println!("{:#?}", nums);
    while l < r {
        let mid = l + (r - l) / 2;
        // println!("{:?}", (l, r, mid));
        if !solve(&nums, nums[mid], k) {
            l = mid + 1
        } else {
            r = mid
        }
    }

    nums[l]
}

// 685_冗余连接_II
// https://leetcode.cn/problems/redundant-connection-ii/description/
// pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {}

// 253. 会议室 II
// https://leetcode.cn/problems/meeting-rooms-ii/description/?envType=weekly-question&envId=2024-04-01
pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
    let mut t = vec![];
    for inter in intervals {
        t.push((inter[0], 1));
        t.push((inter[1], -1));
    }
    // 注意，这里需要多级排序
    t.sort_by(|&a, &b| (a.0).cmp(&b.0).then(a.1.cmp(&b.1)));
    let mut cur_room = 0;
    let mut max_room = 0;

    for i in 0..t.len() {
        cur_room += t[i].1;
        max_room = max_room.max(cur_room);
    }

    max_room
}

// 20. 有效的括号
// https://leetcode.cn/problems/valid-parentheses/description/?envType=study-plan-v2&envId=top-100-liked

pub fn is_valid(s: String) -> bool {
    let mut pair = HashMap::new();
    pair.insert(')', '(');
    pair.insert(']', '[');
    pair.insert('}', '{');

    let mut stk = Vec::new();
    for c in s.chars() {
        match pair.get(&c) {
            Some(&expected_open) => {
                if stk.pop() != Some(expected_open) {
                    return false;
                }
            }
            None => stk.push(c),
        }
    }

    stk.is_empty()
}

// 155. 最小栈
struct MinStack {
    stk: VecDeque<(i32, i32)>, // Each element is a tuple (value, current_min)
}

impl MinStack {
    fn new() -> Self {
        Self {
            stk: VecDeque::new(),
        }
    }

    fn push(&mut self, val: i32) {
        let current_min = if let Some(&(_, min)) = self.stk.back() {
            std::cmp::min(min, val)
        } else {
            val
        };
        self.stk.push_back((val, current_min));
    }

    fn pop(&mut self) {
        self.stk.pop_back();
    }

    fn top(&self) -> i32 {
        self.stk.back().map(|&(val, _)| val).unwrap()
    }

    fn get_min(&self) -> i32 {
        self.stk.back().map(|&(_, min)| min).unwrap()
    }
}

// 394_字符串解码
pub fn decode_string(s: String) -> String {
    let mut num_stack = Vec::new();
    let mut str_stack = Vec::new();
    let mut current_num = 0usize;
    let mut current_str = String::new();

    for c in s.chars() {
        match c {
            '0'..='9' => {
                current_num = current_num * 10 + c.to_digit(10).unwrap() as usize;
            }
            '[' => {
                // Push current number and string to their respective stacks
                num_stack.push(current_num);
                str_stack.push(current_str.clone());
                // Reset for the next segment
                current_num = 0;
                current_str = String::new();
            }
            ']' => {
                let num = num_stack.pop().unwrap();
                let last_str = str_stack.pop().unwrap();
                // repeat `current_str` `num` times and append to `last_str`
                let repeated = current_str.repeat(num);
                current_str = last_str + &repeated;
            }
            _ => current_str.push(c), // Alphabetic and other characters
        }
    }

    current_str
}

// 739. 每日温度
// https://leetcode.cn/problems/daily-temperatures/description/?envType=study-plan-v2&envId=top-100-liked
pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut stack = Vec::new();
    let mut results = vec![0; temperatures.len()];

    for (index, &current_temp) in temperatures.iter().enumerate() {
        while let Some(&top_index) = stack.last() {
            if current_temp <= temperatures[top_index] {
                break;
            }
            let last_index = stack.pop().unwrap();
            results[last_index] = (index - last_index) as i32;
        }
        stack.push(index);
    }

    results
}

// 84. 柱状图中最大的矩形
pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let n = heights.len();

    let mut left_stk = vec![];
    let mut left_bounds = vec![n as i32; n];
    for (idx, &p) in heights.iter().enumerate() {
        while let Some(&i) = left_stk.last() {
            if p >= heights[i as usize] {
                break;
            }
            let last_idx = left_stk.pop().unwrap();
            left_bounds[last_idx as usize] = (idx) as i32;
        }
        left_stk.push(idx as i32);
    }

    let mut right_stk = vec![];
    let mut right_bounds = vec![-1; n];
    for (idx, &p) in heights.iter().rev().enumerate() {
        let idx = n - 1 - idx;
        while let Some(&i) = right_stk.last() {
            if p >= heights[i as usize] {
                break;
            }
            let last_idx = right_stk.pop().unwrap();
            right_bounds[last_idx as usize] = (idx) as i32;
        }
        right_stk.push(idx as i32);
    }

    let mut res = 0;
    for i in 0..n {
        res = res.max(heights[i] * (left_bounds[i] - right_bounds[i] - 1));
    }

    res
}

// 377_组合总和_Ⅳ
pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
    let mut dp = vec![0; target as usize + 1];
    dp[0] = 1;

    for i in 1..=target {
        for &n in nums.iter() {
            if i >= n {
                dp[i as usize] += dp[i as usize - n as usize];
            }
        }
    }

    dp[target as usize]
}

// 215. 数组中的第K个最大元素
use std::collections::BinaryHeap;

pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut min_heap = BinaryHeap::with_capacity(k as usize);

    for num in nums {
        // 将元素以负值形式加入最大堆以模拟最小堆
        let neg_num = -num;

        if min_heap.len() < k as usize {
            min_heap.push(neg_num);
        } else if neg_num < *min_heap.peek().unwrap() {
            min_heap.pop();
            min_heap.push(neg_num);
        }
        println!("{:?}", min_heap);
    }
    // 返回第 k 大的元素（注意取反得到原始值）
    -min_heap.peek().unwrap()
}

// 347. 前 K 个高频元素

// 1052. 爱生气的书店老板
pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
    let mut satisfied = 0;
    let mut max_addition = 0;
    let mut addition = 0;
    for i in 0..customers.len() {
        if grumpy[i] == 0 {
            satisfied += customers[i];
        } else {
            addition += customers[i];
        }
        if i >= minutes as usize && grumpy[i - minutes as usize] == 1 {
            addition -= customers[i - minutes as usize];
        }
        max_addition = max_addition.max(addition);
    }
    satisfied + max_addition
}

// 2739. 总行驶距离
pub fn distance_traveled(mut main_tank: i32, mut additional_tank: i32) -> i32 {
    let mut distance = 0;

    while main_tank > 0 {
        // Calculate the next block of distance covered using the main tank.
        let next_distance = 10 * main_tank.min(5); // Travel up to 50 km
        distance += next_distance;

        // Every 50 km, attempt to transfer fuel from the additional tank to the main tank.
        if main_tank >= 5 {
            if additional_tank > 0 {
                additional_tank -= 1;
                main_tank -= 4; // Net effect: main tank only reduces by 4 because 1 unit is added from the additional tank
            } else {
                main_tank -= 5; // No additional tank fuel available, reduce by 5
            }
        } else {
            main_tank = 0; // If less than 5 units left, they get used up
        }
    }

    distance
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
