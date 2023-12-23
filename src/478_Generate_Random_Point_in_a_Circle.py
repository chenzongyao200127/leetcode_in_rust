# 478_Generate_Random_Point_in_a_Circle
# https://leetcode.cn/problems/generate-random-point-in-a-circle/description/

from typing import List
import math
import random

class Solution:

    def __init__(self, radius: float, x_center: float, y_center: float):
        self.xc = x_center
        self.yc = y_center
        self.r = radius

    def randPoint(self) -> List[float]:
        while True:
            x, y = random.uniform(-self.r, self.r), random.uniform(-self.r, self.r)
            if x * x + y * y <= self.r * self.r:
                return [self.xc + x, self.yc + y]


# Your Solution object will be instantiated and called as such:
# obj = Solution(radius, x_center, y_center)
# param_1 = obj.randPoint()