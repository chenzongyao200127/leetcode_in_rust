// 239_滑动窗口最大值
// https://leetcode.cn/problems/sliding-window-maximum/description/

#include <vector>
#include <deque>
#include <iostream>

std::vector<int> maxSlidingWindow(std::vector<int> &nums, int k)
{
    std::vector<int> result;
    std::deque<int> dq; // 存储索引

    for (int i = 0; i < nums.size(); ++i)
    {
        // 移除不在滑动窗口内的元素的索引
        while (!dq.empty() && dq.front() <= i - k)
        {
            dq.pop_front();
        }

        // 维护deque的单调递减性质，移除所有小于当前元素的索引
        while (!dq.empty() && nums[dq.back()] < nums[i])
        {
            dq.pop_back();
        }

        // 将当前元素索引加入deque
        dq.push_back(i);

        // 如果窗口已经达到大小k，记录当前窗口的最大值
        if (i >= k - 1)
        {
            result.push_back(nums[dq.front()]);
        }
    }

    return result;
}

int main()
{
    std::vector<int> nums = {1, 2, 3, 4, 6};
    int k = 2;
    std::vector<int> maxValues = maxSlidingWindow(nums, k);

    for (int val : maxValues)
    {
        std::cout << val << " ";
    }
    std::cout << std::endl;

    return 0;
}