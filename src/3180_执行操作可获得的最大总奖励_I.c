// 3180_执行操作可获得的最大总奖励_I
// 给你一个整数数组 rewardValues，长度为 n，代表奖励的值。
// 最初，你的总奖励 x 为 0，所有下标都是 未标记 的。你可以执行以下操作 任意次 ：
// 从区间 [0, n - 1] 中选择一个 未标记 的下标 i。
// 如果 rewardValues[i] 大于 你当前的总奖励 x，则将 rewardValues[i] 加到 x 上（即 x = x + rewardValues[i]），并 标记 下标 i。
// 以整数形式返回执行最优操作能够获得的 最大 总奖励。
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Comparison function for qsort
int cmp(const void *a, const void *b)
{
    return *(int *)a - *(int *)b;
}

// Function to calculate the maximum total reward
int maxTotalReward(int *rewardValues, int rewardValuesSize)
{
    // Sort the reward values in ascending order
    qsort(rewardValues, rewardValuesSize, sizeof(int), cmp);

    // Find the maximum reward value
    int maxReward = rewardValues[rewardValuesSize - 1];

    // Allocate memory for the dynamic programming array
    int *dp = (int *)malloc(2 * maxReward * sizeof(int));
    if (dp == NULL)
    {
        perror("Failed to allocate memory");
        return -1;
    }

    // Initialize the dp array
    memset(dp, 0, 2 * maxReward * sizeof(int));
    dp[0] = 1;

    // Fill the dp array
    for (int i = 0; i < rewardValuesSize; i++)
    {
        int reward = rewardValues[i];
        for (int j = 2 * reward - 1; j >= reward; j--)
        {
            if (dp[j - reward] == 1)
            {
                dp[j] = 1;
            }
        }
    }

    // Find the maximum total reward
    int maxTotal = 0;
    for (int i = 0; i < 2 * maxReward; i++)
    {
        if (dp[i] == 1)
        {
            maxTotal = i;
        }
    }

    // Free the allocated memory
    free(dp);

    return maxTotal;
}
