// 322_零钱兑换
// https://leetcode.cn/problems/coin-change/description/?envType=study-plan-v2&envId=top-interview-150

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut dp = vec![amount + 1; amount + 1];
        dp[0] = 0;
        for i in 1..=amount {
            for &coin in coins.iter() {
                if i >= coin as usize {
                    dp[i] = dp[i].min(dp[i - coin as usize] + 1);
                }
            }
        }
        if dp[amount] == amount + 1 {
            -1
        } else {
            dp[amount] as i32
        }
    }
}
