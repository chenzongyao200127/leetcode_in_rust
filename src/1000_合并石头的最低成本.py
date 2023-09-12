# 1000_合并石头的最低成本
# https://leetcode.cn/problems/minimum-cost-to-merge-stones/description/

# 有 n 堆石头排成一排，第 i 堆中有 stones[i] 块石头。

# 每次 移动 需要将 连续的 k 堆石头合并为一堆，而这次移动的成本为这 k 堆中石头的总数。

# 返回把所有石头合并成一堆的最低成本。如果无法合并成一堆，返回 -1 。

 

# 示例 1：
# 输入：stones = [3,2,4,1], K = 2
# 输出：20
# 解释：
# 从 [3, 2, 4, 1] 开始。
# 合并 [3, 2]，成本为 5，剩下 [5, 4, 1]。
# 合并 [4, 1]，成本为 5，剩下 [5, 5]。
# 合并 [5, 5]，成本为 10，剩下 [10]。
# 总成本 20，这是可能的最小值。

# 示例 2：
# 输入：stones = [3,2,4,1], K = 3
# 输出：-1
# 解释：任何合并操作后，都会剩下 2 堆，我们无法再进行合并。所以这项任务是不可能完成的。

# 示例 3：
# 输入：stones = [3,5,1,2,6], K = 3
# 输出：25
# 解释：
# 从 [3, 5, 1, 2, 6] 开始。
# 合并 [5, 1, 2]，成本为 8，剩下 [3, 8, 6]。
# 合并 [3, 8, 6]，成本为 17，剩下 [17]。
# 总成本 25，这是可能的最小值。

from typing import List
from functools import lru_cache

class Solution:
    def mergeStones(self, stones: List[int], k: int) -> int:
        n = len(stones)
        if n == 1:
            return 0

        if (n - 1) % (k - 1) != 0:
            return -1

        # 计算前缀和
        prefix_sum = [0] * (n + 1)
        for i in range(n):
            prefix_sum[i + 1] = prefix_sum[i] + stones[i]

        # 记忆化搜索的辅助函数
        @lru_cache(None)
        def dfs(i: int, j: int) -> int:
            if j - i + 1 < k:
                return 0
            
            cost = 0
            if (j - i) % (k - 1) == 0:
                cost = prefix_sum[j + 1] - prefix_sum[i]

            return cost + min(dfs(i, mid) + dfs(mid + 1, j) for mid in range(i, j, k - 1))

        return dfs(0, n - 1)

        


from typing import List

class Solution:
    def mergeStones(self, stones: List[int], k: int) -> int:
        n = len(stones)
        
        if (n - 1) % (k - 1) != 0:
            return -1

        prefix_sum = [0] * (n + 1)
        for i in range(n):
            prefix_sum[i + 1] = prefix_sum[i] + stones[i]

        def get_range_sum(i, j):
            return prefix_sum[j + 1] - prefix_sum[i]

        memo = {}

        def dfs(i, j, m):
            # 若当前[i, j]之间有m堆石子
            if (j - i + 1 - m) % (k - 1):
                return float('inf')
            
            if (i, j, m) in memo:
                return memo[(i, j, m)]

            if i == j:
                return 0 if m == 1 else float('inf')

            if m == 1:
                memo[(i, j, m)] = dfs(i, j, k) + get_range_sum(i, j)
                return memo[(i, j, m)]

            cost = float('inf')
            for mid in range(i, j, k - 1):
                cost = min(cost, dfs(i, mid, 1) + dfs(mid + 1, j, m - 1))
            
            memo[(i, j, m)] = cost
            return cost
        
        return dfs(0, n - 1, 1)



s = Solution()
ans = s.mergeStones(stones = [3,2,4,1], k = 2)        
print(ans)

s = Solution()
ans = s.mergeStones(stones = [3,2,4,1], k = 3)        
print(ans)

s = Solution()
ans = s.mergeStones(stones = [3,5,1,2,6], k = 3)        
print(ans)



from typing import List
from itertools import accumulate
from functools import cache




class Solution:
    def mergeStones(self, stones: List[int], k: int) -> int:
        n = len(stones)
        
        # 如果石头数减一模(k-1)不为零，那么无法合并成一堆
        if (n - 1) % (k - 1):  
            return -1
        
        # 使用accumulate计算stones的前缀和，存储在s中
        s = list(accumulate(stones, initial=0))
        
        @cache  
        def dfs(i: int, j: int) -> int:
            # 如果i和j指向同一个位置，只有一堆石头，返回0表示无需合并
            if i == j:  
                return 0
            
            # 对于每个可能的中间点m，尝试将石头分为两部分，并计算合并它们的成本
            # (i, m) 和 (m+1, j)是两部分，递归地求解这两部分
            res = min(dfs(i, m) + dfs(m + 1, j) for m in range(i, j, k - 1))
            
            # 如果从i到j的距离是(k-1)的倍数，那么这些石头可以合并成一堆
            # 我们需要增加合并它们的成本
            # ⭐ 这一步没搞懂
            # if (j - i) % (k - 1) == 0:
            if (j - i + 1) == k:
                res += s[j + 1] - s[i]  # 使用前缀和快速计算i到j的总和
            
            return res
        
        return dfs(0, n - 1)


s = Solution()
ans = s.mergeStones(stones = [3,2,4,1], k = 2)        
print(ans)

s = Solution()
ans = s.mergeStones(stones = [3,2,4,1], k = 3)        
print(ans)

s = Solution()
ans = s.mergeStones(stones = [3,5,1,2,6], k = 3)        
print(ans)