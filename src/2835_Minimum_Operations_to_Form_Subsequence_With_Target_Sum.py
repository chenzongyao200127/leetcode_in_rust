# 2835_Minimum_Operations_to_Form_Subsequence_With_Target_Sum
# https://leetcode.cn/problems/minimum-operations-to-form-subsequence-with-target-sum/

# 给你一个下标从 0 开始的数组 nums ，它包含 非负 整数，且全部为 2 的幂，同时给你一个整数 target 。

# 一次操作中，你必须对数组做以下修改：

# 选择数组中一个元素 nums[i] ，满足 nums[i] > 1 。
# 将 nums[i] 从数组中删除。
# 在 nums 的 末尾 添加 两个 数，值都为 nums[i] / 2 。
# 你的目标是让 nums 的一个 子序列 的元素和等于 target ，请你返回达成这一目标的 最少操作次数 。如果无法得到这样的子序列，请你返回 -1 。

# 数组中一个 子序列 是通过删除原数组中一些元素，并且不改变剩余元素顺序得到的剩余数组。
from typing import List
from collections import Counter
class Solution:
    def minOperations(self, nums: List[int], target: int) -> int:
        if sum(nums) < target:
            return -1

        count = Counter(nums)
        operations = total_sum = 0

        for i in range(31):  
            total_sum += count[1 << i] << i
            
            if (target >> i & 1) == 0:
                continue
            
            total_sum -= 1 << i
            
            if total_sum >= 0:
                continue
            
            for j in range(i + 1, 31):
                if count[1 << j]:
                    operations += j - i
                    count[1 << j] -= 1
                    total_sum += 1 << j
                    break

        return operations
                