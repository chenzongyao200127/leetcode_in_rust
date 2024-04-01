// 42_接雨水
// https://leetcode.cn/problems/trapping-rain-water/description/?envType=study-plan-v2&envId=top-100-liked

#include <vector>
#include <stack>
#include <algorithm>
using namespace std;

class Solution
{
    int trap(vector<int> &height)
    {
        stack<int> stk;
        int ans = 0;
        for (int idx = 0; idx < height.size(); ++idx)
        {
            while (!stk.empty() && height[stk.top()] < height[idx])
            {
                int low = stk.top();
                stk.pop();
                if (!stk.empty())
                {
                    int bounded_height = min(height[idx], height[stk.top()]) - height[low];
                    ans += bounded_height * (idx - stk.top() - 1);
                }
            }
            stk.push(idx);
        }
        return ans;
    }
};