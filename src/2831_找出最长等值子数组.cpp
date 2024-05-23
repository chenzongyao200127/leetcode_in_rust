#include <string>
#include <algorithm>
#include <vector>
#include <unordered_map>
using namespace std;

class Solution
{
public:
    int longestEqualSubarray(vector<int> &nums, int k)
    {
        int n = nums.size();
        unordered_map<int, vector<int>> indexes_map;

        // Store indices for each number
        for (int i = 0; i < n; ++i)
        {
            indexes_map[nums[i]].push_back(i);
        }

        int max_equal_length = 0;

        for (auto &p : indexes_map)
        {
            vector<int> &indexes = p.second;
            int j = 0;
            for (int i = 0; i < indexes.size(); ++i)
            {
                // Expand j to maintain the condition
                while (j < indexes.size() && indexes[j] - indexes[i] - (j - i) <= k)
                {
                    j++;
                }
                max_equal_length = max(max_equal_length, j - i);
            }
        }

        return max_equal_length;
    }
};