// 322_Coin_Change
// https://leetcode.cn/problems/coin-change/description/

// 给你一个整数数组 coins ，表示不同面额的硬币；以及一个整数 amount ，表示总金额。

// 计算并返回可以凑成总金额所需的 最少的硬币个数 。如果没有任何一种硬币组合能组成总金额，返回 -1 。

// 你可以认为每种硬币的数量是无限的。

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![amount + 1; amount as usize + 1];
        dp[0] = 0;
        for coin in coins {
            (coin as usize..=amount as usize).for_each(|i| {
                dp[i] = dp[i].min(dp[i - coin as usize] + 1);
            });
        }
        if dp[amount as usize] == amount + 1 {
            -1
        } else {
            dp[amount as usize]
        }
    }
}


impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut dp = Vec::with_capacity(amount + 1);
        let mut min;
        dp.push(0);
        for i in 1..=amount {
            min = amount;
            for &n in coins.iter() {
                if i >= (n as usize) {
                    min = min.min(dp[i - n as usize]);
                }
            }
            dp.push(min + 1);
        }
        
        return if dp[amount] > amount {
            -1
        } else {
            dp[amount] as i32
        };
    }
}

