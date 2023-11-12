// 1_Two_Sum
// https://leetcode.cn/problems/two-sum/description/

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut idx = HashMap::new(); // 创建一个空哈希表
        for (j, &x) in nums.iter().enumerate() { // 枚举 j
            // 在左边找 nums[i]，满足 nums[i]+x=target
            if let Some(&i) = idx.get(&(target - x)) { // 找到了
                return vec![i as i32, j as i32]; // 返回两个数的下标
            }
            idx.insert(x, j); // 保存 nums[j] 和 j
        }
        unreachable!() // 题目保证有解，循环中一定会 return
    }
}
