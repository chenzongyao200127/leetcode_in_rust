#include <vector>
using namespace std;

// 300. 最长递增子序列
// https://leetcode.cn/problems/longest-increasing-subsequence/description/?envType=study-plan-v2&envId=top-interview-150
class Solution
{
public:
    int lengthOfLIS(vector<int> &nums)
    {
        int n = nums.size();
        if (n == 0)
        {
            return 0;
        }
        vector<int> dp(n, 1);
        int res = 1;
        for (int i = 1; i < n; i++)
        {
            for (int j = 0; j < i; j++)
            {
                if (nums[i] > nums[j])
                {
                    dp[i] = max(dp[i], dp[j] + 1);
                }
            }
            res = max(res, dp[i]);
        }
        return res;
    }
};