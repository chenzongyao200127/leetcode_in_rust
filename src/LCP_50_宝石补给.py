# LCP_50_宝石补给
# https://leetcode.cn/problems/WHnhjV/description/

# 示例 1：
# 输入：gem = [3,1,2], operations = [[0,2],[2,1],[2,0]]
# 输出：2
# 解释： 第 1 次操作，勇者 0 将一半的宝石赠送给勇者 2， gem = [2,1,3] 第 2 次操作，勇者 2 将一半的宝石赠送给勇者 1， gem = [2,2,2] 第 3 次操作，勇者 2 将一半的宝石赠送给勇者 0， gem = [3,2,1] 返回 3 - 1 = 2

# 示例 2：
# 输入：gem = [100,0,50,100], operations = [[0,2],[0,1],[3,0],[3,0]]
# 输出：75
# 解释： 第 1 次操作，勇者 0 将一半的宝石赠送给勇者 2， gem = [50,0,100,100] 第 2 次操作，勇者 0 将一半的宝石赠送给勇者 1， gem = [25,25,100,100] 第 3 次操作，勇者 3 将一半的宝石赠送给勇者 0， gem = [75,25,100,50] 第 4 次操作，勇者 3 将一半的宝石赠送给勇者 0， gem = [100,25,100,25] 返回 100 - 25 = 75

# 示例 3：
# 输入：gem = [0,0,0,0], operations = [[1,2],[3,1],[1,2]]
# 输出：0

from typing import List
class Solution:
    def giveGem(self, gem: List[int], operations: List[List[int]]) -> int:
        for op in operations:
            give, rece = op[0], op[1]
            tmp = gem[give] // 2
            gem[give] -= tmp
            gem[rece] += tmp
        
        return max(gem) - min(gem)