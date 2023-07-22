// 446_Arithmetic_Slices_II_-_Subsequence
// https://leetcode.cn/problems/arithmetic-slices-ii-subsequence/description/

use std::collections::HashMap;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let nums: Vec<i64> = nums.iter().map(|&x| x as i64).collect();
        let n = nums.len();
        let mut dp = HashMap::new();
        let mut ans = 0;
        for i in 0..n {
            for j in 0..i {
                let diff = nums[i] - nums[j];
                let count = dp.get(&(j, diff)).unwrap_or(&0).clone();
                dp.entry((i, diff)).and_modify(|x| *x += count + 1).or_insert(count + 1);
                ans += count;
            }
        }
        ans as i32
    }
}