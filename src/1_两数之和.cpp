#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <unordered_set>
#include <unordered_map>
using namespace std;

class Solution
{
public:
    vector<int> twoSum(vector<int> &nums, int target)
    {
        unordered_set<int> set;
        unordered_map<int, int> map;

        for (int i = 0; i < nums.size(); ++i)
        {
            int c = nums[i];
            int t = target - c;
            if (set.find(t) != set.end())
            {
                return vector<int>{i, map[t]};
            }
            else
            {
                set.insert(c);
                map[c] = i;
            }
        }

        throw runtime_error("Not reachable!");
    }
};

class Solution
{
public:
    vector<int> twoSum(vector<int> &nums, int target)
    {
        unordered_map<int, int> hashtable;
        for (int i = 0; i < nums.size(); ++i)
        {
            auto it = hashtable.find(target - nums[i]);
            if (it != hashtable.end())
            {
                return {it->second, i};
            }
            hashtable[nums[i]] = i;
        }
        return {};
    }
};
