// 122_Best_Time_to_Buy_and_Sell_Stock_II
// https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-ii/?envType=daily-question&envId=2023-10-09

#include <stdio.h>
#ifndef __cplusplus
#include <stdbool.h>
#endif

int maxProfit(int* prices, int pricesSize){
    int cur_value = *prices;
    int profit = 0;

    for (int i = 0; i < pricesSize; i++) {
        if (prices[i] > cur_value) {
            profit += (prices[i] - cur_value);
            cur_value = prices[i];
        } 
        if (prices[i] < cur_value) {
            cur_value = prices[i];
        }
    }

    return profit;
}