// 438_找到字符串中所有字母异位词
// https://leetcode.cn/problems/find-all-anagrams-in-a-string/description/?envType=study-plan-v2&envId=top-100-liked

#include <algorithm>
#include <vector>
#include <string>

using namespace std;

class Solution
{
public:
    vector<int> findAnagrams(string s, string p)
    {
        vector<int> ans;
        if (p.length() > s.length())
        {
            return ans;
        }

        vector<int> pCount(26, 0), sCount(26, 0);

        for (int i = 0; i < p.length(); ++i)
        {
            ++pCount[p[i] - 'a'];
            ++sCount[s[i] - 'a'];
        }

        if (sCount == pCount)
        {
            ans.push_back(0);
        }

        for (int i = p.length(); i < s.length(); ++i)
        {
            ++sCount[s[i] - 'a'];
            --sCount[s[i - p.length()] - 'a'];
            if (sCount == pCount)
            {
                ans.push_back(i - p.length() + 1);
            }
        }

        return ans;
    }
};