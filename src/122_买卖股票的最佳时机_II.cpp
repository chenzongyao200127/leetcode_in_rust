// 122. 买卖股票的最佳时机 II
// https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-ii/description/

#include <vector>

using namespace std;

class Solution
{
public:
    int maxProfit(vector<int> &prices)
    {
        int res = 0;
        for (int i = 0; i < prices.size() - 1; i++)
        {
            int profit = prices[i + 1] - prices[i];
            if (profit > 0)
            {
                res += profit;
            }
        }
        return res;
    }
};