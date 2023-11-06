# 1465_切割后面积最大的蛋糕
# https://leetcode.cn/problems/maximum-area-of-a-piece-of-cake-after-horizontal-and-vertical-cuts/description/


from typing import List
class Solution:
    def maxArea(self, h: int, w: int, horizontalCuts: List[int], verticalCuts: List[int]) -> int:
        horizontalCuts.append(0)
        horizontalCuts.append(h)
        
        verticalCuts.append(0)
        verticalCuts.append(w)

        horizontalCuts.sort()
        verticalCuts.sort()
        
        def longestDistant(cuts):
            if len(cuts) == 1:
                return cuts[0]
            else:
                l = 0
                d = 0
                for i in range(len(cuts)):
                    d = max(d, cuts[i] - l)
                    l = cuts[i]
                return d
        
        return longestDistant(horizontalCuts) * longestDistant(verticalCuts) % 1000000007
                
                    
# 代码优化
from typing import List

class Solution:
    def maxArea(self, h: int, w: int, horizontalCuts: List[int], verticalCuts: List[int]) -> int:
        MOD = 10**9 + 7
        
        horizontalCuts = sorted([0] + horizontalCuts + [h])
        verticalCuts = sorted([0] + verticalCuts + [w])
        
        max_h = max(horizontalCuts[i+1] - horizontalCuts[i] for i in range(len(horizontalCuts) - 1))
        max_v = max(verticalCuts[i+1] - verticalCuts[i] for i in range(len(verticalCuts) - 1))
        
        return (max_h * max_v) % MOD
