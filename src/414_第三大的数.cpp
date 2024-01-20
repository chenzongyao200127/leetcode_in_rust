#include <vector>  // 用于支持 vector
#include <climits> // 用于 LONG_MIN

using namespace std;

class Solution
{
public:
    int thirdMax(vector<int> &nums)
    {
        long a = LONG_MIN, b = LONG_MIN, c = LONG_MIN;

        for (long num : nums)
        {
            if (num == a || num == b || num == c)
                continue; // Skip duplicates

            if (num > a)
            {
                c = b;
                b = a;
                a = num;
            }
            else if (num > b)
            {
                c = b;
                b = num;
            }
            else if (num > c)
            {
                c = num;
            }
        }

        return c == LONG_MIN ? a : c;
    }
};
