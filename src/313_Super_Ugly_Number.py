# 313_Super_Ugly_Number
# https://leetcode.cn/problems/super-ugly-number/

# 超级丑数 是一个正整数，并满足其所有质因数都出现在质数数组 primes 中。
# 给你一个整数 n 和一个整数数组 primes ，返回第 n 个 超级丑数 。
# 题目数据保证第 n 个 超级丑数 在 32-bit 带符号整数范围内。

# 输入：n = 12, primes = [2,7,13,19]
# 输出：32 
# 解释：给定长度为 4 的质数数组 primes = [2,7,13,19]，前 12 个超级丑数序列为：[1,2,4,7,8,13,14,16,19,26,28,32] 。
from typing import List
import heapq

class Solution:
    # 定义函数 nthSuperUglyNumber，接受两个参数，n 和 primes。n 表示要求得第 n 个超级丑数，primes 是给定的质数列表。
    def nthSuperUglyNumber(self, n: int, primes: List[int]) -> int:
        # 初始化 factors 为给定的质数列表。
        factors = primes
        # 初始化 seen 集合，用于存储已经看到的丑数，避免重复计算。
        seen = {1}
        # 初始化 heap 列表，其中只包含一个丑数 1。heap 是一个最小堆，用于存储丑数，最小的丑数始终在堆顶。
        heap = [1]

        # 执行 n - 1 次循环。每次循环都会找到一个新的丑数，并将其添加到堆中。
        for i in range(n - 1):
            # 从堆中取出最小的丑数 curr。
            curr = heapq.heappop(heap)
            # 对于 factors 列表中的每个质数，都尝试生成一个新的丑数，并将其添加到堆中。
            for factor in factors:
                # 计算可能的新丑数 nxt。如果 nxt 已经在 seen 集合中，则不再进行处理；否则，将其添加到 seen 集合和堆中。
                if (nxt := curr * factor) not in seen:
                    seen.add(nxt)
                    heapq.heappush(heap, nxt)

        # 循环结束后，堆顶的元素就是第 n 个丑数。将其从堆中取出并返回。
        return heapq.heappop(heap)


class Solution:
    def nthSuperUglyNumber(self, n: int, primes: List[int]) -> int:
        dp = [0] * (n + 1)
        m = len(primes)
        pointers = [0] * m
        nums = [1] * m

        for i in range(1, n + 1):
            min_num = min(nums)
            dp[i] = min_num
            for j in range(m):
                if nums[j] == min_num:
                    pointers[j] += 1
                    nums[j] = dp[pointers[j]] * primes[j]
        
        return dp[n]
