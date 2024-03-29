# 822. Card Flipping Game
# https://leetcode.cn/problems/card-flipping-game/

# 在桌子上有 n 张卡片，每张卡片的正面和背面都写着一个正数（正面与背面上的数有可能不一样）。
# 我们可以先翻转任意张卡片，然后选择其中一张卡片。
# 如果选中的那张卡片背面的数字 x 与任意一张卡片的正面的数字都不同，那么这个数字是我们想要的数字。
# 哪个数是这些想要的数字中最小的数（找到这些数中的最小值）呢？如果没有一个数字符合要求的，输出 0 。
# 其中, fronts[i] 和 backs[i] 分别代表第 i 张卡片的正面和背面的数字。
# 如果我们通过翻转卡片来交换正面与背面上的数，那么当初在正面的数就变成背面的数，背面的数就变成正面的数。

from typing import List
class Solution:
    def flipgame(self, fronts: List[int], backs: List[int]) -> int:
        n = len(fronts)
        s = set()
        order = []
        for i in range(n):
            if fronts[i] == backs[i]:
                s.add(fronts[i])
            else:
                order.append(fronts[i])
                order.append(backs[i])
        
        order.sort()
        for num in order:
            if num in s:
                continue
            else:
                return num

        return 0
                
        
        
# 写法优化
class Solution:
    def flipgame(self, fronts: List[int], backs: List[int]) -> int:
        forbidden = {x for x, y in zip(fronts, backs) if x == y}
        return min((x for x in fronts + backs if x not in forbidden), default=0)

        