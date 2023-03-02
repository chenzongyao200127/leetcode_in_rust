// 122. Best Time to Buy and Sell Stock II
// https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-ii/

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut new_prices = prices.clone();
        for i in 1..prices.len()-1 {
            if ((prices[i-1] < prices[i]) && (prices[i] < prices[i+1])) || ((prices[i-1] > prices[i]) && (prices[i] > prices[i+1])) {
                new_prices[i] = -1;
            }
        }
        let new_price: Vec<_> = new_prices.iter().filter(|&x| *x != -1).collect();
        // println!("{:?}", new_price);
        for i in 1..new_price.len() {
            let add = new_price[i] - new_price[i-1];
            if add >= 0 {
                ans += add;
            }
        }
        
        ans
    }
}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
      let mut max_profit = 0;
        if prices.len() < 2 {
            return max_profit;
        }
        let mut prev_price = prices[0];
        for i in 1..prices.len() {
            if prices[i] > prev_price {
                max_profit += prices[i] - prev_price;
            }
            prev_price = prices[i];
        }
        max_profit

    }
}
