from collections import defaultdict
from typing import List
from functools import lru_cache


# 给你一个整数数组 nums 和 两个 整数 l 和 r。你的任务是找到一个长度在 l 和 r 之间（包含）且和大于 0 的 子数组 的 最小 和。

# 返回满足条件的子数组的 最小 和。如果不存在这样的子数组，则返回 -1。

# 子数组 是数组中的一个连续 非空 元素序列。

class Solution:
    def minimumSumSubarray(self, nums: List[int], l: int, r: int) -> int:
        min_sum = float('inf')
        n = len(nums)

        for i in range(n):
            for j in range(i, n):
                if l <= (j - i + 1) <= r:
                    current_sum = sum(nums[i:j+1])
                    if current_sum > 0:
                        min_sum = min(min_sum, current_sum)

        return min_sum if min_sum != float('inf') else -1


# 给你两个字符串 s 和 t（它们互为字母异位词），以及一个整数 k。

# 你的任务是判断是否可以将字符串 s 分割成 k 个等长的子字符串，然后重新排列这些子字符串，并以任意顺序连接它们，使得最终得到的新字符串与给定的字符串 t 相匹配。

# 如果可以做到，返回 true；否则，返回 false。

# 字母异位词 是指由另一个单词或短语的所有字母重新排列形成的单词或短语，使用所有原始字母恰好一次。

# 子字符串 是字符串中的一个连续 非空 字符序列。


class Solution:
    def isPossibleToRearrange(self, s: str, t: str, k: int) -> bool:
        if len(s) % k != 0:
            return False

        # split s into k parts
        n = len(s)
        part_len = n // k
        parts = []
        for i in range(0, n, part_len):
            parts.append(s[i:i+part_len])

        # split t into k parts
        m = len(t)
        part_len = m // k
        parts_t = []
        for i in range(0, m, part_len):
            parts_t.append(t[i:i+part_len])

        # sort parts and parts_t
        sorted_parts = sorted(parts)
        sorted_parts_t = sorted(parts_t)

        return sorted_parts == sorted_parts_t


# 给你一个整数数组 nums 和三个整数 k、op1 和 op2。

# 你可以对 nums 执行以下操作：

# 操作 1：选择一个下标 i，将 nums[i] 除以 2，并 向上取整 到最接近的整数。你最多可以执行此操作 op1 次，并且每个下标最多只能执行一次。
# 操作 2：选择一个下标 i，仅当 nums[i] 大于或等于 k 时，从 nums[i] 中减去 k。你最多可以执行此操作 op2 次，并且每个下标最多只能执行一次。

# 注意： 两种操作可以应用于同一下标，但每种操作最多只能应用一次。
class Solution:
    def minArraySum(self, nums: List[int], k: int, op1: int, op2: int) -> int:

        @lru_cache(None)
        def dfs(index, op1_left, op2_left):
            if index == len(nums):
                return 0
            if op1_left == 0 and op2_left == 0:
                return sum(nums[index:])

            current_num = nums[index]
            min_sum = float('inf')

            # Option 1: Do not perform any operation
            min_sum = min(min_sum, current_num +
                          dfs(index + 1, op1_left, op2_left))

            # Option 2: Perform operation 1
            if op1_left > 0:
                new_num = (current_num + 1) // 2
                min_sum = min(min_sum, new_num +
                              dfs(index + 1, op1_left - 1, op2_left))

            # Option 3: Perform operation 2
            if op2_left > 0 and current_num >= k:
                new_num = current_num - k
                min_sum = min(min_sum, new_num +
                              dfs(index + 1, op1_left, op2_left - 1))

            # Option 4: Perform both operations (op1 first, then op2)
            if op1_left > 0 and op2_left > 0 and current_num >= k:
                new_num = (current_num + 1) // 2
                if new_num >= k:
                    new_num = new_num - k
                    min_sum = min(min_sum, new_num + dfs(index +
                                                         1, op1_left - 1, op2_left - 1))

            # Option 5: Perform both operations (op2 first, then op1)
            if op1_left > 0 and op2_left > 0 and current_num >= k:
                new_num = current_num - k
                new_num = (new_num + 1) // 2
                min_sum = min(min_sum, new_num + dfs(index +
                              1, op1_left - 1, op2_left - 1))

            return min_sum

        return dfs(0, op1, op2)


# s = Solution()
# print(s.minArraySum(nums=[2, 8, 3, 19, 3], k=3, op1=1, op2=1))
# print(s.minArraySum(nums=[2, 4, 3], k=3, op1=2, op2=1))
# print(s.minArraySum(nums=[2, 0], k=2, op1=1, op2=2))


# 存在一棵具有 n 个节点的无向树，节点编号为 0 到 n - 1。给你一个长度为 n - 1 的二维整数数组 edges，其中 edges[i] = [ui, vi, wi] 表示在树中节点 ui 和 vi 之间有一条权重为 wi 的边。

# 你的任务是移除零条或多条边，使得：

# 每个节点与至多 k 个其他节点有边直接相连，其中 k 是给定的输入。
# 剩余边的权重之和 最大化 。
# 返回在进行必要的移除后，剩余边的权重的 最大 可能和。


class Solution:
    def maximizeSumOfWeights(self, edges: List[List[int]], k: int) -> int:
        # Build adjacency list
        adj = defaultdict(list)
        for u, v, w in edges:
            adj[u].append((v, w))
            adj[v].append((u, w))

        # Initialize dp for each node, [0] is the max sum with no extra node included, [1] is the max with one extra node
        dp = {u: [0, 0] for u in adj}

        def dfs(u, p):
            # List to hold contributions from children nodes
            contributions = []

            # Traverse through all neighbors of u
            for v, w in adj[u]:
                if v != p:  # Avoid visiting parent node
                    dfs(v, u)  # Recursively call dfs for the child node
                    # Contribution is calculated as the weight + the difference of dp[v][1] and dp[v][0]
                    contrib = (w + dp[v][1]) - dp[v][0]
                    contributions.append(contrib)

            # Sort contributions in descending order to pick the highest contributing nodes first
            contributions_sorted = sorted(contributions, reverse=True)

            # Calculate dp[u][0] (max sum without including any extra node from this subtree)
            t0 = min(k, len(contributions_sorted))
            # Sum of dp[v][1] for the top t0 contributions
            sum_keep = sum(dp[v][1] for v, _ in adj[u][:t0])
            # Sum of dp[v][0] for the remaining contributions
            sum_not_keep = sum(dp[v][0] for v, _ in adj[u][t0:])
            dp[u][0] = sum_keep + sum_not_keep

            # Calculate dp[u][1] (max sum with one extra node from this subtree)
            # One less because we are taking one extra node
            t1 = min(k - 1, len(contributions_sorted))
            sum_keep = 0
            for i in range(t1):
                # Use contributions sorted
                sum_keep += contributions_sorted[i] + dp[adj[u][i][1]][1]
            sum_not_keep = 0
            for i in range(t1, len(contributions_sorted)):
                sum_keep += dp[adj[u][i][1]][1]
                sum_not_keep += dp[adj[u][i][0]]

            dp[u][1] = sum_keep + sum_not_keep

        # The answer is dp[root][0] which is the max sum without any extra node from the subtree
        root = 0  # Assuming root is node 0, you can change this to any valid root node in your graph
        dfs(root, -1)

        return dp[root][0]


# Test the function
s = Solution()
print(s.maximizeSumOfWeights(edges=[[0, 1, 14], [0, 3, 44], [3, 2, 37]], k=1))
