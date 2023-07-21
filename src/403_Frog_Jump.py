# 403_Frog_Jump
# https://leetcode.cn/problems/frog-jump/

# 一只青蛙想要过河。 假定河流被等分为若干个单元格，并且在每一个单元格内都有可能放有一块石子（也有可能没有）。 
# 青蛙可以跳上石子，但是不可以跳入水中。

# 给你石子的位置列表 stones（用单元格序号 升序 表示）， 请判定青蛙能否成功过河（即能否在最后一步跳至最后一块石子上）。
# 开始时， 青蛙默认已站在第一块石子上，并可以假定它第一步只能跳跃 1 个单位（即只能从单元格 1 跳至单元格 2 ）。

# 如果青蛙上一步跳跃了 k 个单位，那么它接下来的跳跃距离只能选择为 k - 1、k 或 k + 1 个单位。 另请注意，青蛙只能向前方（终点的方向）跳跃。

# 输入：stones = [0,1,3,5,6,8,12,17]
# 输出：true
# 解释：青蛙可以成功过河，按照如下方案跳跃：跳 1 个单位到第 2 块石子, 然后跳 2 个单位到第 3 块石子, 接着 跳 2 个单位到第 4 块石子, 
# 然后跳 3 个单位到第 6 块石子, 跳 4 个单位到第 7 块石子, 最后，跳 5 个单位到第 8 个石子（即最后一块石子）。

from typing import List
from functools import cache

# 回溯
class Solution:
    def canCross(self, stones: List[int]) -> bool:
        stone_set = set(stones)
        
        @cache
        def dfs(cur_loca, pre_jump):
            if cur_loca == stones[-1]:
                return True
            
            for j in range(pre_jump-1, pre_jump+2):
                if j > 0 and (cur_loca + j) in stone_set:
                    if dfs(cur_loca + j, j):
                        return True
                    
            return False
        
        return dfs(0,0)

# 在计算机科学中，NP-hard（非确定性多项式难度）是复杂度理论中的问题分类。
# 如果问题A的解决方案可以在多项式时间内用来验证问题B的解决方案，那么我们就说问题B是问题A的多项式时间归约，或者说问题A至少与问题B一样"难"。
# 如果一个问题至少和NP中的每一个问题一样难，那么我们就说这个问题是NP-hard的。

# NP-hard的问题可以在非确定性多项式时间内验证解的正确性，但找到解可能需要非多项式时间（例如，可能需要指数时间）。
# 因此，这类问题在实际中往往是无法在合理时间内解决的。

# 判断一个问题是否为NP-hard问题通常需要数学证明。
# 常见的方法是找到一个已知的NP-hard问题，并证明它可以在多项式时间内归约到你正在考虑的问题。
# 这样，你就证明了你的问题至少和一个已知的NP-hard问题一样难，从而证明它是NP-hard的。

# 这是一种非常抽象的概念，但可以帮助我们理解某些问题的困难性，并指导我们设计和选择算法。
# 比如，如果我们知道一个问题是NP-hard的，那么我们就不应该期待找到一个能够在所有情况下都能在多项式时间内找到最优解的算法。
# 我们可能需要使用一些启发式算法，比如遗传算法或模拟退火，来找到"足够好"的解，而不是最优解。
            
from typing import List

class Solution:
    def canCross(self, stones: List[int]) -> bool:
        if stones[1] != 1:
            return False

        n = len(stones)
        dp = [[False] * n for _ in range(n)]
        dp[0][0] = True
        dp[1][1] = True

        for i in range(2, n):
            for j in range(1, i):
                k = stones[i] - stones[j]
                if k < 0 or k >= n:
                    continue
                dp[i][k] |= dp[j][k]
                if k - 1 >= 0:
                    dp[i][k] |= dp[j][k - 1]
                if k + 1 < n:
                    dp[i][k] |= dp[j][k + 1]

        return any(dp[-1])

# 优化：
# 你的代码已经使用了动态规划进行优化，但还可以进一步改进空间复杂度和时间复杂度。

# 首先，当前的dp数组的大小为n*n，其中n为石头的数量。
# 实际上，青蛙每一次跳跃，跳跃的步数最大也只能增加1。
# 因此，对于第i个石头，青蛙到达它的步数最大不会超过i，我们可以利用这一点将dp的空间复杂度从O(n^2)优化为O(n)。

# 其次，在每一轮迭代中，你都检查了所有小于当前石头的石头，时间复杂度是O(n^2)。
# 实际上，由于每次跳跃的步数不能比上一次跳跃的步数多1，所以青蛙不能跳过太远的石头。
# 对于每个石头，我们可以存储一个集合来记录能够跳到这个石头的所有步数，这样在后续的石头中就可以快速找到能够跳到当前石头的石头。

# 以下是优化后的代码：
from typing import List

class Solution:
    def canCross(self, stones: List[int]) -> bool:
        if stones[1] != 1:
            return False

        stone_positions = {stone: set() for stone in stones}
        stone_positions[0].add(0)

        for stone in stones:
            for k in stone_positions[stone]:
                for step in range(k - 1, k + 2):
                    if step > 0 and stone + step in stone_positions:
                        stone_positions[stone + step].add(step)

        return len(stone_positions[stones[-1]]) > 0

# 这个版本的代码首先初始化了一个字典stone_positions，键是石头的位置，值是一个集合，表示能够跳到这个石头的所有步数。
# 然后，通过迭代每个石头和每个步数，更新stone_positions。如果步数大于0，并且可以跳到下一个石头，就将步数添加到下一个石头的集合中。
# 最后，检查能否跳到最后一块石头，如果能，返回True，否则返回False。
# 这个版本的代码的时间复杂度和空间复杂度都是O(n)，比原始的代码更加高效。