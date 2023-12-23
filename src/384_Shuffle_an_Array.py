# 384_Shuffle_an_Array
# https://leetcode.cn/problems/shuffle-an-array/description/
from typing import List
import random

class Solution:

    def __init__(self, nums: List[int]):
        self.nums = nums

    def reset(self) -> List[int]:
        return self.nums

    def shuffle(self) -> List[int]:
        self.temp = list(self.nums)
        for i in range(len(self.nums)):
            idx = random.randint(i, len(self.nums) - 1)
            self.temp[i], self.temp[idx] = self.temp[idx], self.temp[i]
        return self.temp
    
# Your Solution object will be instantiated and called as such:
# obj = Solution(nums)
# param_1 = obj.reset()
# param_2 = obj.shuffle()