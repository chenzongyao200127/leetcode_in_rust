// 121. Best Time to Buy and Sell Stock
// https://leetcode.cn/problems/best-time-to-buy-and-sell-stock/

// DP
// 动态规划 前i天的最大收益 = max{前i-1天的最大收益，第i天的价格-前i-1天中的最小价格}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut dp = vec![0; prices.len()];
        let mut min = prices[0];
        for i in 1..prices.len() {
            min = min.min(prices[i]);
            dp[i] = dp[i-1].max(prices[i] - min);
        }

        dp[dp.len()-1]
    }
}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut minmin = i32::MAX;
        let mut maxsell = 0;
        for price in prices.iter() {
            if *price < minmin {
                minmin = *price;
            }
            if price - minmin > maxsell {
                maxsell = price - minmin;
            }
        }
        return maxsell;
    }
}
