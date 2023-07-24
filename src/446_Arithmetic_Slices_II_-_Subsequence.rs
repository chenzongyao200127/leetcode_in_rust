// 446_Arithmetic_Slices_II_-_Subsequence
// https://leetcode.cn/problems/arithmetic-slices-ii-subsequence/description/

use std::collections::HashMap;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        //将nums中的元素转换为i64类型
        let nums: Vec<i64> = nums.iter().map(|&x| x as i64).collect();
        let n = nums.len();
        //使用HashMap存储子序列的差值和个数
        let mut dp = HashMap::new();
        let mut ans = 0;

        //枚举i和j，计算以i结尾、差值为diff的等差数列个数
        for i in 0..n {
            for j in 0..i {
                let diff = nums[i] - nums[j];
                //获取以j结尾、差值为diff的等差数列个数，若不存在则设为0
                let count = dp.get(&(j, diff)).unwrap_or(&0).clone();
                //将以i结尾、差值为diff的等差数列个数加上以j结尾、差值为diff的等差数列个数，并更新HashMap
                dp.entry((i, diff)).and_modify(|x| *x += count + 1).or_insert(count + 1);
                //累加以j结尾、差值为diff的等差数列个数
                ans += count;
            }
        }
        //返回等差数列个数
        ans as i32
    }
}