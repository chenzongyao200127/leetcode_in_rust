#include <vector>    // 用于支持 vector
#include <algorithm> // 用于 max 函数

class Solution
{
public:
    int findMaxConsecutiveOnes(std::vector<int> &nums)
    {
        int res = 0;
        int tmp = 0;

        for (int n : nums)
        {
            if (n == 1)
            {
                tmp += 1;
            }
            else
            {
                res = std::max(res, tmp);
                tmp = 0;
            }
        }

        res = std::max(res, tmp);

        return res;
    }
};
