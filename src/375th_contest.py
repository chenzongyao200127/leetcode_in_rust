# 374th_contest

from typing import List

class Solution:
    def countTestedDevices(self, batteryPercentages: List[int]) -> int:
        tested_devices = 0

        for i in range(len(batteryPercentages)):
            if batteryPercentages[i] > 0:
                tested_devices += 1
                for j in range(i + 1, len(batteryPercentages)):
                    batteryPercentages[j] = max(0, batteryPercentages[j] - 1)

        return tested_devices


class Solution:
    def getGoodIndices(self, variables: List[List[int]], target: int) -> List[int]:
        good_indices = []
        
        for i, (ai, bi, ci, mi) in enumerate(variables):
            if ((ai ** bi) % 10) ** ci % mi == target:
                good_indices.append(i)

        return good_indices



class Solution:
    def countSubarrays(self, nums: List[int], k: int) -> int:
        overall_max_element = max(nums)
        n = len(nums)
        total_count = 0
        start = 0
        max_element_count = 0

        for end in range(n):

            if nums[end] == overall_max_element:
                max_element_count += 1

            while max_element_count >= k:
                total_count += (n - end)
                if nums[start] == overall_max_element:
                    max_element_count -= 1
                start += 1

        return total_count


from collections import defaultdict
from typing import List

def merge(intervals):
    intervals.sort()
    merged = []
    for start, end in intervals:
        if not merged or start > merged[-1][1]:
            merged.append([start, end])
        else:
            merged[-1][1] = max(merged[-1][1], end)
    return merged

class Solution:
    def numberOfGoodPartitions(self, nums: List[int]) -> int:
        num_to_indices = defaultdict(list)
        for index, num in enumerate(nums):
            num_to_indices[num].append(index)

        intervals = [[indices[0], indices[-1]] for indices in num_to_indices.values()]

        merged_intervals = merge(intervals)
        
        MOD = 10**9 + 7
        return pow(2, len(merged_intervals) - 1, MOD)
