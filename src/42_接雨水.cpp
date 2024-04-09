// 42_接雨水
// https://leetcode.cn/problems/trapping-rain-water/description/?envType=study-plan-v2&envId=top-100-liked

#include <vector>
#include <stack>
#include <algorithm>
using namespace std;

// 单调栈写法
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

// 双指针
#include <vector>
#include <algorithm> // For std::max and std::min

class Solution
{
public:
    int trap(const std::vector<int> &height)
    {
        std::size_t n = height.size();
        if (n == 0)
        { // Handle empty vector case
            return 0;
        }

        std::vector<int> leftMax(n);
        leftMax[0] = height[0];
        for (std::size_t i = 1; i < n; ++i)
        {
            leftMax[i] = std::max(leftMax[i - 1], height[i]);
        }

        std::vector<int> rightMax(n);
        rightMax[n - 1] = height[n - 1];
        for (std::size_t i = n - 1; i-- > 0;)
        {
            rightMax[i] = std::max(rightMax[i + 1], height[i]);
        }

        int res = 0;
        for (std::size_t i = 0; i < n; ++i)
        {
            res += std::min(rightMax[i], leftMax[i]) - height[i];
        }

        return res;
    }
};