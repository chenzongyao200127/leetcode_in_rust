// 377_Combination_Sum_IV
// https://leetcode.cn/problems/combination-sum-iv/

// 给你一个由 不同 整数组成的数组 nums ，和一个目标整数 target 。请你从 nums 中找出并返回总和为 target 的元素组合的个数。

// 题目数据保证答案符合 32 位整数范围。

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0; target as usize + 1];
        dp[0] = 1;

        for i in 1..=target {
            for n in nums.iter() {
                if i >= *n {
                    dp[i as usize] += dp[i as usize - *n as usize];
                }
            }
        }

        dp[target as usize]
    }
}