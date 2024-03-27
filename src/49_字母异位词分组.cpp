// 49_字母异位词分组
// https://leetcode.cn/problems/group-anagrams/description/?envType=study-plan-v2&envId=top-100-liked

#include <vector>
#include <string>
#include <algorithm>
#include <unordered_map>
#include <iostream>

class Solution
{
public:
    std::vector<std::vector<std::string>> groupAnagrams(const std::vector<std::string> &strs)
    {
        std::unordered_map<std::string, std::vector<std::string>> map;
        for (const std::string &s : strs)
        {
            std::string key = s;
            std::sort(key.begin(), key.end());
            map[key].push_back(s);
        }

        std::vector<std::vector<std::string>> grouped;
        for (auto &pair : map)
        {
            grouped.push_back(std::move(pair.second));
        }

        return grouped;
    }
};

using namespace std;
class Solution
{
public:
    vector<vector<string>>
    groupAnagrams(vector<string> &strs)
    {
        std::unordered_map<std::string, std::vector<string>> mapping;
        for (uint32_t i = 0; i < strs.size(); ++i)
        {
            std::string a = strs[i];
            std::sort(a.begin(), a.end());
            mapping[a].emplace_back(std::move(strs[i]));
        }

        vector<vector<string>> result;
        result.reserve(strs.size());
        for (auto &it : mapping)
        {
            result.emplace_back(std::move(it.second));
        }
        return std::move(result);
    }
};