# 2681_Power_of_Heroes
# https://leetcode.cn/problems/power-of-heroes/

# 给你一个下标从 0 开始的整数数组 nums ，它表示英雄的能力值。如果我们选出一部分英雄，这组英雄的 力量 定义为：

# i0 ，i1 ，... ik 表示这组英雄在数组中的下标。那么这组英雄的力量
# 为 max(nums[i0],nums[i1] ... nums[ik])2 * min(nums[i0],nums[i1] ... nums[ik]) 。
# 请你返回所有可能的 非空 英雄组的 力量 之和。由于答案可能非常大，请你将结果对 109 + 7 取余。

# 输入：nums = [2,1,4]
# 输出：141
# 解释：
# 第 1 组：[2] 的力量为 22 * 2 = 8 。
# 第 2 组：[1] 的力量为 12 * 1 = 1 。
# 第 3 组：[4] 的力量为 42 * 4 = 64 。
# 第 4 组：[2,1] 的力量为 22 * 1 = 4 。
# 第 5 组：[2,4] 的力量为 42 * 2 = 32 。
# 第 6 组：[1,4] 的力量为 42 * 1 = 16 。
# 第​ ​​​​​​7 组：[2,1,4] 的力量为 42​​​​​​​ * 1 = 16 。
# 所有英雄组的力量之和为 8 + 1 + 64 + 4 + 32 + 16 + 16 = 141 。


# Try something with sorting the array.
# For a pair of array elements nums[i] and nums[j] (i < j), 
#   the power would be nums[i]*nums[j]^2 regardless of how many elements in between are included.
# The number of subsets with the above as power will correspond to 2^(j-i-1).
# Try collecting the terms for nums[0], nums[1], …, nums[j-1] 
#   when computing the power of heroes ending at index j to get the power in a single pass.
from typing import List
class Solution:
    def sumOfPower(self, nums: List[int]) -> int:
        nums.sort()
        dp = [0 for i in range(len(nums))]
        pre_sum = [0 for i in range(len(nums) + 1)]
        res, mod = 0, 10 ** 9 + 7
        for i in range(len(nums)):
            dp[i] = (nums[i] + pre_sum[i]) % mod
            pre_sum[i + 1] = (pre_sum[i] + dp[i]) % mod
            res = (res + nums[i] * nums[i] * dp[i]) % mod
        return res



class Solution:
    def sumOfPower(self, nums: List[int]) -> int:
        MOD = 10 ** 9 + 7
        nums.sort()
        ans = s = 0
        for x in nums:  # x 作为最大值
            ans = (ans + x * x * (x + s)) % MOD
            s = (s * 2 + x) % MOD  # 递推计算下一个 s
        return ans