// https://leetcode.cn/problems/target-sum/submissions/
int findTargetSumWays(int *nums, int numsSize, int target)
{
    int sum = 0;
    for (int i = 0; i < numsSize; i++)
    {
        sum += nums[i];
    }
    int diff = sum - target;
    if (diff < 0 || diff % 2 != 0)
    {
        return 0;
    }
    int n = numsSize, neg = diff / 2;
    int dp[n + 1][neg + 1];
    memset(dp, 0, sizeof(dp));
    dp[0][0] = 1;
    for (int i = 1; i <= n; i++)
    {
        int num = nums[i - 1];
        for (int j = 0; j <= neg; j++)
        {
            dp[i][j] = dp[i - 1][j];
            if (j >= num)
            {
                dp[i][j] += dp[i - 1][j - num];
            }
        }
    }
    return dp[n][neg];
}

// 作者：LeetCode-Solution
// 链接：https://leetcode.cn/problems/target-sum/solution/mu-biao-he-by-leetcode-solution-o0cp/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。