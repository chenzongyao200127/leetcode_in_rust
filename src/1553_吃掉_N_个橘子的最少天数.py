# 1553_吃掉_N_个橘子的最少天数
# https://leetcode.cn/problems/minimum-number-of-days-to-eat-n-oranges/description/?envType=daily-question&envId=2024-05-12

# 厨房里总共有 n 个橘子，你决定每一天选择如下方式之一吃这些橘子：

# 吃掉一个橘子。
# 如果剩余橘子数 n 能被 2 整除，那么你可以吃掉 n/2 个橘子。
# 如果剩余橘子数 n 能被 3 整除，那么你可以吃掉 2*(n/3) 个橘子。
# 每天你只能从以上 3 种方案中选择一种方案。

# 请你返回吃掉所有 n 个橘子的最少天数。
from heapq import heappop, heappush
from math import inf
from heapq import heappush, heappop
from collections import defaultdict
from typing import inf
from functools import cache


class Solution:
    def minDays(self, n: int) -> int:
        # 记忆化递归
        memo = {}

        @cache
        def dfs(n: int) -> int:
            if n == 0:
                return 0
            if n == 1:
                return 1
            if n in memo:
                return memo[n]
            # 操作1：吃掉一个橘子
            res_2 = res_3 = float('inf')
            res_1 = 1 + dfs(n - 1)
            # 操作2：吃掉 n/2 个橘子
            if n % 2 == 0:
                res_2 = 1 + dfs(n // 2)
            # 操作3：吃掉 2*(n/3) 个橘子
            if n % 3 == 0:
                res_3 = 1 + dfs(n // 3)
            res = min(res_1, res_2, res_3)
            # print(f'n={n}, res={res}')
            return res
        return dfs(n)


# add test cases
# n = 10
s = Solution()
print(s.minDays(10))  # 4
# n = 6
s = Solution()
print(s.minDays(6))  # 3

# use DP to rewrite


class Solution:
    def minDays(self, n: int) -> int:
        # DP table
        dp = [0, 1] + [float('inf')] * (n - 1)
        for i in range(2, n + 1):
            # 操作1：吃掉一个橘子
            res_1 = 1 + dp[i - 1]
            # 操作2：吃掉 n/2 个橘子
            res_2 = res_3 = float('inf')
            if i % 2 == 0:
                res_2 = 1 + dp[i // 2]
            # 操作3：吃掉 2*(n/3) 个橘子
            if i % 3 == 0:
                res_3 = 1 + dp[i // 3]
            dp[i] = min(res_1, res_2, res_3)
        return dp[n]


# add test cases
# n = 9459568
s = Solution()
print(s.minDays(9459568))  # 21


class Solution:
    def minDays(self, n: int) -> int:
        @cache  # 缓存装饰器，避免重复计算 dfs 的结果（记忆化）
        def dfs(i: int) -> int:
            if i <= 1:
                return i
            return min(dfs(i // 2) + i % 2, dfs(i // 3) + i % 3) + 1
        return dfs(n)

# Dijkstra 算法


class Solution:
    def minDays(self, n: int) -> int:
        dis = defaultdict(lambda: inf)
        h = [(0, n)]
        while True:
            dx, x = heappop(h)
            if x <= 1:
                return dx + x
            if dx > dis[x]:
                continue
            for d in 2, 3:
                y = x // d
                dy = dx + x % d + 1
                if dy < dis[y]:
                    dis[y] = dy
                    heappush(h, (dy, y))


class Solution:
    def minDays(self, n: int) -> int:
        # Dictionary to store the minimum days to reach each number
        min_days = defaultdict(lambda: inf)

        # Priority queue to process numbers based on the minimum days required
        heap = [(0, n)]

        while heap:
            # Extract the number with the current minimum days
            days, number = heappop(heap)

            # If the number is 1 or less, return the days required to reduce it to zero
            if number <= 1:
                return days + number

            # Skip processing if we have already found a quicker way to this number
            if days > min_days[number]:
                continue

            # Explore the possible next steps
            for divisor in (2, 3):
                # Calculate the next number after division
                next_number = number // divisor

                # Calculate days required to reach next_number
                # Including days for leftover parts and one more day for the division operation
                total_days = days + number % divisor + 1

                # Update if a quicker way to reach next_number is found
                if total_days < min_days[next_number]:
                    min_days[next_number] = total_days
                    heappush(heap, (total_days, next_number))

# Example usage
# sol = Solution()
# print(sol.minDays(10))  # Output could be the minimum days to reduce 10 to 0
