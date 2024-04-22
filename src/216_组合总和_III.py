# 216_组合总和_III
# https://leetcode.cn/problems/combination-sum-iii/description/

from typing import List


class Solution:
    def combinationSum3(self, k: int, n: int) -> List[List[int]]:

        def backtrack(start, remaining, path):
            if remaining == 0 and len(path) == k:
                result.append(path.copy())
                return
            elif len(path) == k:
                return

            for i in range(start, 10):
                if i > remaining:
                    break
                path.append(i)
                backtrack(i + 1, remaining - i, path)
                path.pop()

        result = []
        backtrack(1, n, [])
        return result


def maxLengthSubsequence(a: List[int], m: int) -> int:
    n = len(a)
    prefix_sum = [0] * (n + 1)
    for i in range(1, n + 1):
        prefix_sum[i] = prefix_sum[i-1] + a[i-1]

    dp = [1] * n
    ans = 1
    for i in range(1, n):
        for j in range(i):
            if (prefix_sum[i+1] - prefix_sum[j]) >= m * (i - j + 1):
                dp[i] = max(dp[i], dp[j] + 1)
        ans = max(ans, dp[i])

    return ans


# 示例使用
n = 5
m = 5
a = [9, 1, 9, 1, 9]
print(maxLengthSubsequence(a, m))  # 测试函数
