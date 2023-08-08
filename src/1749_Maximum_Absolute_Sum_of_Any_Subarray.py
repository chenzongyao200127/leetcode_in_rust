# 1749_Maximum_Absolute_Sum_of_Any_Subarray
# https://leetcode.cn/problems/maximum-absolute-sum-of-any-subarray/

# 给你一个整数数组 nums 。一个子数组 [numsl, numsl+1, ..., numsr-1, numsr] 的 
# 和的绝对值 为 abs(numsl + numsl+1 + ... + numsr-1 + numsr) 。

# 请你找出 nums 中 和的绝对值 最大的任意子数组（可能为空），并返回该 最大值 。

# abs(x) 定义如下：

# 如果 x 是负整数，那么 abs(x) = -x 。
# 如果 x 是非负整数，那么 abs(x) = x 。

# 示例 1：
# 输入：nums = [1,-3,2,3,-4]
# 输出：5
# 解释：子数组 [2,3] 和的绝对值最大，为 abs(2+3) = abs(5) = 5 。

# 示例 2：
# 输入：nums = [2,-5,1,-4,3,-2]
# 输出：8
# 解释：子数组 [-5,1,-4] 和的绝对值最大，为 abs(-5+1-4) = abs(-8) = 8 。

from typing import List
class Solution:
    def maxAbsoluteSum(self, nums: List[int]) -> int:
        pre_sum_arr = [0]
        pre_sum = 0
        for n in nums:
            pre_sum += n
            pre_sum_arr.append(pre_sum)
        p_max = 0
        n_max = 0
        ans = 0
        for i in range(len(nums)+1):
            if pre_sum_arr[i] > p_max:
                p_max = pre_sum_arr[i]
                ans = max(ans, abs(p_max - n_max))
            elif pre_sum_arr[i] < n_max:
                n_max = pre_sum_arr[i]
                ans = max(ans, abs(p_max - n_max))
            else:
                continue
        return ans
    
    
# 代码优化：
from typing import List

class Solution:
    def maxAbsoluteSum(self, nums: List[int]) -> int:
        pre_sum, p_max, n_max, ans = 0, 0, 0, 0
        
        for n in nums:
            pre_sum += n
            ans = max(ans, abs(pre_sum - p_max), abs(pre_sum - n_max))
            p_max = max(p_max, pre_sum)
            n_max = min(n_max, pre_sum)
        
        return ans
