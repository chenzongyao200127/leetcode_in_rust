#include <iostream>
#include <vector>

using namespace std;

class Solution
{
public:
    int numTrees(int n)
    {
        vector<int> dp(n + 1, 0);
        // dp[i]表示i个节点可能构成的二叉搜索树的个数
        dp[0] = 1;
        dp[1] = 1;
        dp[2] = 2;
        if (n == 2)
            return dp[2];
        // 递推公式
        for (int i = 3; i <= n; i++)
        {
            for (int j = 0; j < i; j++)
            {
                dp[i] += dp[j] * dp[i - j - 1];
            }
        }
        return dp[n];
    }
};

class Solution
{
public:
    int numTrees(int n)
    {
        vector<int> G(n + 1, 0);
        G[0] = 1;
        G[1] = 1;

        for (int i = 2; i <= n; ++i)
        {
            for (int j = 1; j <= i; ++j)
            {
                G[i] += G[j - 1] * G[i - j];
            }
        }
        return G[n];
    }
};

// add test cases
int main()
{
    Solution solution;
    std::cout << solution.numTrees(1) << std::endl;
    return 0;
}