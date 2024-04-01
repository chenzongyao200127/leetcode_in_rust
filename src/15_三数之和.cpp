#include <vector>
#include <unordered_set>
#include <algorithm>

using namespace std;

class Solution
{
public:
    vector<vector<int>> threeSum(vector<int> &nums)
    {
        int n = nums.size();
        vector<vector<int>> v;
        sort(nums.begin(), nums.end());

        for (int i = 0; i < n - 2; ++i)
        {
            // Handle duplicates for the 'i' element
            if (i > 0 && nums[i] == nums[i - 1])
                continue;

            int l = i + 1, r = n - 1;
            while (l < r)
            {
                int sum = nums[i] + nums[l] + nums[r];
                if (sum < 0)
                {
                    ++l; // Move the left pointer to the right
                }
                else if (sum > 0)
                {
                    --r; // Move the right pointer to the left
                }
                else
                {
                    v.push_back({nums[i], nums[l], nums[r]});
                    // Move the left and right pointers and handle duplicates for 'l' and 'r'
                    while (l < r && nums[l] == nums[l + 1])
                        ++l;
                    while (l < r && nums[r] == nums[r - 1])
                        --r;
                    ++l;
                    --r;
                }
            }
        }
        return v;
    }
};