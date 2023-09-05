// 312_Burst_Balloons
// https://leetcode.cn/problems/burst-balloons/description/

// 有 n 个气球，编号为0 到 n - 1，每个气球上都标有一个数字，这些数字存在数组 nums 中。

// 现在要求你戳破所有的气球。戳破第 i 个气球，你可以获得 nums[i - 1] * nums[i] * nums[i + 1] 枚硬币。 
// 这里的 i - 1 和 i + 1 代表和 i 相邻的两个气球的序号。如果 i - 1或 i + 1 超出了数组的边界，那么就当它是一个数字为 1 的气球。

// 求所能获得硬币的最大数量。

use::std::cmp::max;
impl Solution {
    pub fn max_coins(mut nums: Vec<i32>) -> i32 {
        let mut new_nums = vec![1];
        new_nums.append(&mut nums);
        new_nums.push(1);
        let l = new_nums.len();

        let mut dp = vec![vec![0; l]; l];

        for gap in 2..l {
            for i in 0..l-gap {
                let j = i + gap;
                for k in i+1..j {
                    dp[i][j] = max(dp[i][j], dp[i][k] + dp[k][j] + new_nums[i] * new_nums[k] * new_nums[j]);
                }
            }
        }

        dp[0][l-1]
    }
}