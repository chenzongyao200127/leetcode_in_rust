// 518_Coin_Change_II
// https://leetcode.cn/problems/coin-change-ii/description/

// 给你一个整数数组 coins 表示不同面额的硬币，另给一个整数 amount 表示总金额。
// 请你计算并返回可以凑成总金额的硬币组合数。如果任何硬币组合都无法凑出总金额，返回 0 。
// 假设每一种面额的硬币有无限个。 
// 题目数据保证结果符合 32 位带符号整数。

impl Solution {
    pub fn change(amount: i32, mut coins: Vec<i32>) -> i32 {
        if amount == 0 {
            return 0;
        }

        let mut dp = vec![vec![0; amount as usize + 1]; coins.len()];
        coins.sort_unstable();

        for i in 0..dp.len() {
            dp[i][0] = 1;
        }

        for a in 1..=amount as usize {
            if a % coins[0] as usize == 0 {
                dp[0][a] += 1;
            }
        }
    
        for i in 1..coins.len() {
            for a in 1..=amount as usize {
                if a < coins[i] as usize {
                    dp[i][a] = dp[i-1][a];
                }  else {
                    dp[i][a] = dp[i-1][a] + dp[i][a - coins[i] as usize];
                }
            }
        }
    
        dp[coins.len()-1][amount as usize]
    }
}


// 优化
impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut dp = vec![0; amount as usize + 1];
        dp[0] = 1;
        for coin in coins {
            (coin as usize..=amount as usize).for_each(|i| {
                dp[i] += dp[i - coin as usize];
            });
        }
        dp[amount as usize]
    }
}
