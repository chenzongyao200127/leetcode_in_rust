// 2810. 故障键盘
// https://leetcode.cn/problems/faulty-keyboard/description/

#include <string>
#include <algorithm>
using namespace std;

class Solution
{
public:
    string finalString(string s)
    {
        string ans = "";
        for (auto c : s)
        {
            if (c == 'i')
            {
                reverse(ans.begin(), ans.end());
            }
            else
            {
                ans += c;
            }
        }

        return ans;
    }
};