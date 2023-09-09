# 473_Matchsticks_to_Square
# https://leetcode.cn/problems/matchsticks-to-square/description/

# 你将得到一个整数数组 matchsticks ，其中 matchsticks[i] 是第 i 个火柴棒的长度。
# 你要用 所有的火柴棍 拼成一个正方形。你 不能折断 任何一根火柴棒，但你可以把它们连在一起，而且每根火柴棒必须 使用一次 。
# 如果你能使这个正方形，则返回 true ，否则返回 false 。

# 超时
from typing import List
class Solution:
    def makesquare(self, matchsticks: List[int]) -> bool:
        boxes = [0] * 4
        total = sum(matchsticks)
        target = total / 4
        
        if total % 4 != 0:
            return False
        
        def bt(boxes, idx):
            flag = False
            print(boxes)
            
            if idx == len(matchsticks):
                if boxes == [target] * 4:
                    return True
                else:
                    return False
                
            for j in range(4):
                if boxes[j] + matchsticks[idx] <= target:
                    boxes[j] += matchsticks[idx]
                    flag = flag or bt(boxes, idx + 1)
                    if flag == True:
                        return True
                    boxes[j] -= matchsticks[idx]
            
            return False
        
        return bt(boxes, 0)
    

# 记忆化搜索优化 仍然超出内存限制

# 为了减少搜索量，需要对火柴长度从大到小进行排序。⭐
from typing import List, Tuple

class Solution:
    def makesquare(self, matchsticks: List[int]) -> bool:
        matchsticks.sort(reverse=True)
        
        boxes = [0] * 4
        total = sum(matchsticks)
        target = total / 4
        
        if total % 4 != 0:
            return False

        memo = {}

        def bt(boxes: Tuple[int], idx: int) -> bool:
            key = (boxes, idx)
            if key in memo:
                return memo[key]

            if idx == len(matchsticks):
                if boxes == (target, target, target, target):
                    memo[key] = True
                    return True
                else:
                    memo[key] = False
                    return False
                
            for j in range(4):
                if boxes[j] + matchsticks[idx] <= target:
                    new_boxes = list(boxes)
                    new_boxes[j] += matchsticks[idx]
                    if bt(tuple(new_boxes), idx + 1):
                        memo[key] = True
                        return True

            memo[key] = False
            return False
        
        return bt(tuple(boxes), 0)


# 官解
class Solution:
    def makesquare(self, matchsticks: List[int]) -> bool:
        totalLen = sum(matchsticks)
        if totalLen % 4:
            return False
        matchsticks.sort(reverse=True)

        edges = [0] * 4
        def dfs(idx: int) -> bool:
            if idx == len(matchsticks):
                return True
            for i in range(4):
                edges[i] += matchsticks[idx]
                if edges[i] <= totalLen // 4 and dfs(idx + 1):
                    return True
                edges[i] -= matchsticks[idx]
            return False
        return dfs(0)

# 使用区间动态规划（Dynamic Programming）优化此问题需要一个完全不同的思路。
# 这是一个分割等和子集的变种，也是一个很固定的问题，但我们需要将其变化为四个子集而不是两个。
# 这里提供一个基于状态压缩的动态规划的方法来优化：

# 1. `matchsticks`的数量不会超过16，我们可以利用二进制表示来描述一个子集。
# 例如，如果 `matchsticks` 为 [2, 3, 4, 5]，子集 [2, 4] 可以被表示为 1010。

# 2. 我们可以用 `dp` 数组来保存中间结果。
# `dp[mask]` 表示是否可以从 `matchsticks` 中选择一个子集，使其和为 `target` 的整数倍，且总个数不超过4。

from typing import List

class Solution:
    def makesquare(self, matchsticks: List[int]) -> bool:
        # 计算所有火柴的总长度
        totalLen = sum(matchsticks)

        # 如果火柴的总长度不能被4整除，那么它们不能形成一个正方形
        if totalLen % 4:
            return False

        # 计算正方形的每条边的期望长度
        tLen = totalLen // 4

        # 使用DP方法，其中dp[s]代表从火柴集合中选择的子集，s用二进制位表示。
        # dp[s]的值是这个子集的总长度模tLen（正方形的一条边）的结果。
        # 初始状态是0，意味着没有选择任何火柴
        dp = [-1] * (1 << len(matchsticks))
        dp[0] = 0

        # 遍历所有可能的子集
        for s in range(1, len(dp)):
            # 遍历所有火柴
            for k, v in enumerate(matchsticks):
                # 如果s的第k位是0，表示在子集s中没有选择第k根火柴
                if s & (1 << k) == 0:
                    continue
                
                # s1是不包括第k根火柴的子集
                s1 = s & ~(1 << k)
                
                # 如果s1是一个有效的子集并且加上第k根火柴后的总长度不超过tLen
                if dp[s1] >= 0 and dp[s1] + v <= tLen:
                    # 更新dp[s]的值
                    dp[s] = (dp[s1] + v) % tLen
                    break
                
        # 如果dp的最后一个值（代表所有火柴都被选择的情况）是0，那么火柴可以形成正方形
        return dp[-1] == 0
    
    
s = Solution()
ans = s.makesquare(matchsticks = [1,1,2,2,2])
print(ans)

s = Solution()
ans = s.makesquare(matchsticks = [3,3,3,3,4])
print(ans)

s = Solution()
ans = s.makesquare(matchsticks = [5969561,8742425,2513572,3352059,9084275,2194427,1017540,2324577,6810719,8936380,7868365,2755770,9954463,9912280,4713511])
print(ans)

s = Solution()
ans = s.makesquare(matchsticks = [2,2,2,2,2,6])
print(ans)


