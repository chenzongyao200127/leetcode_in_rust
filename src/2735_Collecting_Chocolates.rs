// 2735. 收集巧克力
// https://leetcode.cn/problems/collecting-chocolates/description/

use std::collections::HashMap;

impl Solution {
    pub fn min_cost(nums: Vec<i32>, x: i32) -> i64 {
        let n = nums.len();

        let mut cumulative_costs: Vec<i64> = (0..n).map(|i| i as i64 * x as i64).collect();

        for start in 0..n {
            let mut current_min = nums[start];

            for end in start..(n + start) {
                current_min = current_min.min(nums[end % n]);
                cumulative_costs[end - start] += current_min as i64;
            }
        }

        *cumulative_costs.iter().min().unwrap()
    }
}
