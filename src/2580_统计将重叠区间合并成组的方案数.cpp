#include <vector>
#include <algorithm> // Include this for std::sort

class Solution
{
public:
    int countWays(std::vector<std::vector<int>> &ranges)
    {
        // Sort the vector of vectors based on the start points of the ranges
        std::sort(ranges.begin(), ranges.end(), [](const std::vector<int> &a, const std::vector<int> &b)
                  { return a[0] < b[0]; });

        const int MOD = 1'000'000'007;
        int ans = 1;
        int max_r = -1;
        for (auto &p : ranges)
        {
            if (p[0] > max_r)
            {
                // Found a non-overlapping interval, multiply the answer by 2
                ans = static_cast<long long>(ans) * 2 % MOD; // Use long long to avoid overflow
            }
            // Extend the current range to include the new interval
            max_r = std::max(max_r, p[1]);
        }
        return ans;
    }
};