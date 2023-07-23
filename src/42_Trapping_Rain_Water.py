# 42_Trapping_Rain_Water
# https://leetcode.cn/problems/trapping-rain-water/

# 给定 n 个非负整数表示每个宽度为 1 的柱子的高度图，计算按此排列的柱子，下雨之后能接多少雨水。

# 输入：height = [0,1,0,2,1,0,1,3,2,1,2,1]
# 输出：6
# 解释：上面是由数组 [0,1,0,2,1,0,1,3,2,1,2,1] 表示的高度图，在这种情况下，可以接 6 个单位的雨水（蓝色部分表示雨水）。 

from typing import List
class Solution:
    def trap(self, height: List[int]) -> int:
        stack = []
        ans = 0
        for i, h in enumerate(height):
            if stack is None:
                stack.append([i,h])
                continue
            while len(stack) and h > stack[-1][1]:
                pop_first = stack.pop()
                if len(stack) >= 1:
                    cur_peak = stack[-1]
                    l = min(cur_peak[1], h) - pop_first[1]
                    r = i - cur_peak[0] - 1
                    ans += l * r
            stack.append([i,h])

        return ans
            
            
solution = Solution()
print(solution.trap(height = [0,1,0,2,1,0,1,3,2,1,2,1]))