// 121_Best_Time_to_Buy_and_Sell_Stock
// https://leetcode.cn/problems/best-time-to-buy-and-sell-stock/description/?envType=daily-question&envId=2023-10-01

#include <stdio.h>
#ifndef __cplusplus
#include <stdbool.h>
#endif

#include <stdio.h>

// 超时
// int maxProfit(int* prices, int pricesSize){
//     int res = 0;
//     if (pricesSize == 1) {
//         return res;
//     }
    
//     for (int i = pricesSize - 1; i > 0; i--) {
//         for (int j = i - 1; j >= 0; j--) {
//             if (prices[i] > prices[j]) {
//                 int temp = prices[i] - prices[j];
//                 if (temp > res) {
//                     res = temp;
//                 }
//             }
//         }
//     }

//     return res;
// }

#define MAX(a,b) (((a)>(b))?(a):(b))
#define MIN(a,b) (((a)<(b))?(a):(b))

int maxProfit(int* prices, int pricesSize){
    int res = 0;
    if (pricesSize == 1) {
        return res;
    }
    
    int m = 0;
    int n = 10000;
    for (int i = 0; i < pricesSize; i++) {
        if (prices[i] > m) {
            m = prices[i];
            res = MAX(res, m - n);
        }
        if (prices[i] < n) {
            n = prices[i];
            m = n;
        }
    }

    return res;
}

void test_maxProfit() {
    int prices1[] = {7,1,5,3,6,4};
    int prices2[] = {7,6,4,3,1};
    int prices3[] = {3,2,6,5,0,3};

    printf("Test 1 result: %d (Expected: 5)\n", maxProfit(prices1, 6));
    printf("Test 2 result: %d (Expected: 0)\n", maxProfit(prices2, 5));
    printf("Test 3 result: %d (Expected: 4)\n", maxProfit(prices3, 6));
}

int main() {
    test_maxProfit();
    return 0;
}
