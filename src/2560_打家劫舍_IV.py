# 2560_打家劫舍_IV
# https://leetcode.cn/problems/house-robber-iv/?envType=daily-question&envId=2023-09-19

# 沿街有一排连续的房屋。每间房屋内都藏有一定的现金。现在有一位小偷计划从这些房屋中窃取现金。
# 由于相邻的房屋装有相互连通的防盗系统，所以小偷 不会窃取相邻的房屋 。
# 小偷的 窃取能力 定义为他在窃取过程中能从单间房屋中窃取的 最大金额 。
# 给你一个整数数组 nums 表示每间房屋存放的现金金额。形式上，从左起第 i 间房屋中放有 nums[i] 美元。
# 另给你一个整数 k ，表示窃贼将会窃取的 最少 房屋数。小偷总能窃取至少 k 间房屋。
# 返回小偷的 最小 窃取能力。

# 示例 1：
# 输入：nums = [2,3,5,9], k = 2
# 输出：5
# 解释：
# 小偷窃取至少 2 间房屋，共有 3 种方式：
# - 窃取下标 0 和 2 处的房屋，窃取能力为 max(nums[0], nums[2]) = 5 。
# - 窃取下标 0 和 3 处的房屋，窃取能力为 max(nums[0], nums[3]) = 9 。
# - 窃取下标 1 和 3 处的房屋，窃取能力为 max(nums[1], nums[3]) = 9 。
# 因此，返回 min(5, 9, 9) = 5 。

# 示例 2：
# 输入：nums = [2,7,9,3,1], k = 2
# 输出：2
# 解释：共有 7 种窃取方式。窃取能力最小的情况所对应的方式是窃取下标 0 和 4 处的房屋。返回 max(nums[0], nums[4]) = 2 。

# def check(select):
#     f0, f1 = 0, 0
#     for n in nums:
#         if n < select:
#             f0 = f1
#         else:
#             f0, f1 = f1, max(f1, f0 + 1)
#     return max(f1, f0 + 1) >= k

from bisect import bisect_left, bisect_right
from typing import List


class Solution:
    def minCapability(self, nums: List[int], k: int) -> int:
        candidate = sorted(nums)
        l = 0
        r = len(candidate) - 1
        mid = l + (r - l) // 2

        def solve(mx: int) -> bool:
            # cnt = i = 0
            # while i < len(nums):
            #     if nums[i] > mx:  # 不偷
            #         i += 1
            #     else:  # 立刻偷
            #         cnt += 1
            #         i += 2  # 跳过下一间房子
            # return cnt >= k

            f0, f1 = 0, 0
            for n in nums:
                if n > mx:
                    f0 = f1
                else:
                    f0, f1 = f1, max(f1, f0 + 1)
            return f1 >= k

        while l < r:
            print(l, r, mid)
            if not solve(candidate[mid]):
                l = mid + 1
            else:
                r = mid
            mid = l + (r - l) // 2

        return candidate[l]


# s = Solution()
# ans = s.minCapability(nums=[2, 3, 5, 9], k=2)
# print(ans)

# s = Solution()
# ans = s.minCapability([2, 7, 9, 3, 1], k=2)
# print(ans)

arr = [1, 1, 4, 5, 6, 6, 6, 7, 8, 12, 56]
ans = bisect_left(arr, 6)
print(ans)

# Return the index where to insert item x in list a, assuming a is sorted.
# The return value i is such that all e in a[:i] have e <= x, and all e in
# a[i:] have e > x. So if x already appears in the list, a.insert(i, x) will insert just after the rightmost x already there.
# Optional args lo (default 0) and hi (default len(a)) bound the slice of a to be searched.
ans = bisect_right(arr, 6)

print(ans)
