# 42_接雨水
# https://leetcode.cn/problems/trapping-rain-water/description/?envType=study-plan-v2&envId=top-100-liked

from typing import List

# 单调栈


class Solution:
    def trap(self, height: List[int]) -> int:
        stk = []
        ans = 0
        n = len(height)
        for i in range(n):
            while stk and height[stk[-1]] < height[i]:
                cur_idx = stk.pop()
                if stk:
                    # 注意，这里的宽度计算方式 (i - stk[-1] - 1)
                    ans += (min(height[i], height[stk[-1]]) -
                            height[cur_idx]) * (i - stk[-1] - 1)
            stk.append(i)

        return ans


# s = Solution()
# ans = s.trap([4, 2, 0, 3, 2, 5])
# assert ans == 9


# 双指针写法
# 首先从左往右遍历数组，记录每一个位置左边（包含自己）的最大值，然后再从右往左遍历数组，记录每一个位置右边（包含自己）的最大值。
# 两次遍历完之后我们就知道每一个位置左边和右边的最大值了，左右两边的最大值围成的区域可以看作是一个桶，
# 桶的高度取决于这两个值的最小值，知道桶的高度就可以计算当前位置所能容纳的水了，最后只需要计算所有位置容纳的水量即可。
class Solution:
    def trap(self, height: List[int]) -> int:
        n = len(height)
        leftMax = height.copy()
        for i in range(1, n):
            leftMax[i] = max(leftMax[i-1], height[i])

        rightMax = height.copy()
        for i in range(n-2, -1, -1):
            rightMax[i] = max(rightMax[i+1], height[i])

        ans = 0
        for i in range(n):
            ans += min(leftMax[i], rightMax[i]) - height[i]

        return ans
# 双指针比单调栈好理解
# 关键是记录每一个位置左边/右边（包含自己）的最大值


s = Solution()
ans = s.trap([4, 2, 0, 3, 2, 5])
assert ans == 9
