# 2831_找出最长等值子数组
# https://leetcode.cn/problems/find-the-longest-equal-subarray/

from typing import List


class Solution:
    def longestEqualSubarray(self, nums: List[int], k: int) -> int:
        n = len(nums)

        # Indexes list stores indices for each unique number in nums.
        indexes_list = [[] for _ in range(n + 1)]

        for idx, num in enumerate(nums):
            indexes_list[num].append(idx)

        max_equal_length = 0

        for indexes in indexes_list:
            if len(indexes):
                j = 0
                for i in range(len(indexes)):
                    # Find the farthest index j such that the number of gaps
                    # between i and j is not more than k.
                    while j < len(indexes) and indexes[j] - indexes[i] - (j - i) <= k:
                        j += 1
                    max_equal_length = max(max_equal_length, j - i)

        return max_equal_length
