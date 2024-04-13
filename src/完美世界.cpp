// 完美世界游戏 0413 C++服务端开发工程师（25届完美实习生）1.5 h
// 编程题2道（一共30分，要求使用C++）单选20题 （40分）多选10题（30分）....不知道为啥有这么多选择题，好多都不会...C++基础太烂了
// 题目 1: 最接近中位数的元素
// 给定一个整数数组 damages 和一个整数 num，找出数组中值最接近中位数的 num 个元素。如果两个元素到中位数的距离相同，则优先返回较大的元素。
// 题目 2: 精确跳跃游戏
// 给定一个整数数组 canStand 和两个整数 minStep 与 maxStep。`canStand[i]` 可以是 1 或者 0，代表在位置 i 是否可以落脚。从数组的第一个元素出发，每次跳跃的距离可以在 minStep 到 maxStep 之间选择。判断是否能够精确地落在数组的最后一个位置上。返回一个布尔值，表示是否能够精确地落在数组的最后一个位置上。
// 第一个题目需要精确计算中位数并根据距离进行排序，而第二个题目则是一个变种的跳跃游戏，需要精确控制跳到数组的最后一个位置。（用贪心最后只过了85%, 想改dp 但是没时间）

#include <algorithm>
#include <cstddef>
#include <cstdio>
#include <vector>

class Solution
{
public:
    /**
     * 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
     *
     *
     * @param damages int整型vector
     * @param num int整型
     * @return int整型vector
     */
    std::vector<int> GetSkills(std::vector<int> &damages, int num)
    {
        // write code here
        int k = num;
        return closestToMedian(damages, k);
    }

private:
    std::vector<int> closestToMedian(std::vector<int> &nums, int k)
    {
        double median = findMedian(nums);
        std::vector<std::pair<int, double>> diff;

        for (int num : nums)
        {
            diff.push_back({num, std::abs(num - median)});
        }

        std::sort(diff.begin(), diff.end(), [](const std::pair<int, double> &a, const std::pair<int, double> &b)
                  {
            if (a.second == b.second) return a.first > b.first;
            return a.second < b.second; });

        std::vector<int> res;
        for (int i = 0; i < k; i++)
        {
            res.push_back(diff[i].first);
        }

        return res;
    }
    double findMedian(std::vector<int> &nums)
    {
        size_t n = nums.size();
        std::nth_element(nums.begin(), nums.begin() + n / 2, nums.end());

        if (n % 2 == 0)
        {
            int mid1 = nums[n / 2];
            int mid2 = *std::max_element(nums.begin(), nums.begin() + n / 2);
            return (mid1 + mid2) / 2.0;
        }
        else
        {
            return nums[n / 2] * 1.0;
        }
    }
};

#include <queue>
#include <vector>
class Solution
{
public:
    /**
     * 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
     *
     *
     * @param canStand int整型vector
     * @param minStep int整型
     * @param maxStep int整型
     * @return bool布尔型
     */
    bool CanArrive(std::vector<int> &canStand, int minStep, int maxStep)
    {
        // write code here
        int n = canStand.size();

        if (n == 1)
        {
            return true;
        }

        int currentF = 0;
        int nextF = 0;

        for (int i = 0; i < n && i <= currentF; ++i)
        {
            if (canStand[i] == 1)
            {
                nextF = std::max(nextF, i + maxStep);
                if (nextF >= n - 1)
                {
                    if (i + minStep <= n - 1 && i + maxStep >= n - 1)
                    {
                        return true;
                    }
                }
            }

            if (i == currentF)
            {
                if (currentF < nextF)
                {
                    currentF = nextF;
                }
                else
                {
                    return false;
                }
            }
        }

        return false;
    }
};

#include <vector>
#include <algorithm>

class Solution
{
public:
    bool CanArrive(std::vector<int> &canStand, int minStep, int maxStep)
    {
        int n = canStand.size();
        if (n == 1)
            return true; // If only one element, we are already there.

        std::vector<bool> dp(n, false);
        dp[0] = canStand[0] == 1; // Start from the first position if it's landable.

        for (int i = 0; i < n; ++i)
        {
            if (!dp[i])
                continue; // If we can't reach position i, skip it.

            // Try to jump from i to any position within the jump range that's landable
            for (int j = std::max(i + minStep, 1); j <= std::min(i + maxStep, n - 1); ++j)
            {
                if (canStand[j])
                {
                    dp[j] = true;
                    if (j == n - 1)
                        return true; // If we reach the last position
                }
            }
        }

        return dp[n - 1]; // Return true if we can land on the last position.
    }
};