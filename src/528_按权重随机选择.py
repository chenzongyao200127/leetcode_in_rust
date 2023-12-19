from typing import List
import random

class Solution:
    def __init__(self, w: List[int]):
        self.cum_weights = []
        cumulative = 0
        for weight in w:
            cumulative += weight
            self.cum_weights.append(cumulative)
        print(self.cum_weights)

    def pickIndex(self) -> int:
        # 生成一个位于 0 和权重总和之间的随机数
        target = random.uniform(0, self.cum_weights[-1])
        # 使用二分查找来确定 target 落在哪个区间内
        low, high = 0, len(self.cum_weights)
        while low < high:
            mid = (low + high) // 2
            if target > self.cum_weights[mid]:
                low = mid + 1
            else:
                high = mid
        return low


import bisect
class Solution:

    def __init__(self, w: List[int]):
        t = sum(w)
        self.w = [i/t for i in w]
        self.w = list(accumulate(self.w))


    def pickIndex(self) -> int:
        a = random.random()
        return bisect.bisect(self.w, a)



# Your Solution object will be instantiated and called as such:
# obj = Solution(w)
# param_1 = obj.pickIndex()


# Your Solution object will be instantiated and called as such:
# obj = Solution(w)
# param_1 = obj.pickIndex()

# import random

# def weighted_random_choice(items, weights):
#     return random.choices(items, weights, k=1)[0]

# # 测试
# items = ['a', 'b', 'c', 'd']
# weights = [10, 20, 30, 40]

# # 随机选择一个元素
# selected_item = weighted_random_choice(items, weights)
# print(selected_item)
