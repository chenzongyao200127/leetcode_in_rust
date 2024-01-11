// 239_滑动窗口最大值
// https://leetcode.cn/problems/sliding-window-maximum/description/

use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;
        let mut ans = Vec::new();
        let mut deque = VecDeque::new();

        for i in 0..n {
            // 移除超出窗口范围的元素
            while !deque.is_empty() && deque.front().unwrap() < &(i as i32 - k as i32 + 1) {
                deque.pop_front();
            }

            // 移除比当前元素小的元素，因为它们不可能成为最大值
            while !deque.is_empty() && nums[*deque.back().unwrap() as usize] < nums[i] {
                deque.pop_back();
            }

            deque.push_back(i as i32);

            if i >= k - 1 {
                ans.push(nums[*deque.front().unwrap() as usize]);
            }
        }

        ans
    }
}
