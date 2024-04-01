// 3_无重复字符的最长子串
// https://leetcode.cn/problems/longest-substring-without-repeating-characters/description/?envType=study-plan-v2&envId=top-100-liked

#include <string>
#include <unordered_set>

using namespace std;
class Solution
{
public:
    int lengthOfLongestSubstring(string s)
    {
        unordered_set<char> occ;
        int n = s.size();
        int rk = 0, ans = 0;
        for (int i = 0; i < n; ++i)
        {
            if (i != 0)
            {
                occ.erase(s[i - 1]);
            }
            while (rk < n && !occ.count(s[rk]))
            {
                occ.insert(s[rk]);
                ++rk;
            }
            ans = max(ans, rk - i);
        }
        return ans;
    }
};

class Solution
{
public:
    int lengthOfLongestSubstring(string s)
    {
        int n = s.length(), ans = 0, left = 0;
        unordered_set<char> window; // 维护从下标 left 到下标 right 的字符
        for (int right = 0; right < n; ++right)
        {
            char c = s[right];
            while (window.count(c))
                window.erase(s[left++]);
            window.insert(c);
            ans = max(ans, right - left + 1);
        }
        return ans;
    }
};
