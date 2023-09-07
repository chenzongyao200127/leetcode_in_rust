# 2594. Minimum Time to Repair Cars
# https://leetcode.cn/problems/minimum-time-to-repair-cars/


import math
from typing import List

class Solution:
    def repairCars(self, ranks: List[int], cars: int) -> int:
        def can_fixed(time):
            tmp = 0
            for r in ranks:
                tmp += math.floor(math.sqrt(time // r))
            return tmp >= cars
    
        ranks.sort()
        
        l = 0
        r = ranks[0] * cars * cars + 1
        while l < r:
            mid = (l + r) // 2
            if can_fixed(mid):
                r = mid
            else:
                l = mid + 1
        
        return l



# 在你的原始代码中，你对`r`使用了`mid - 1`来更新它。这在某些情况下可能会跳过正确答案。

# 考虑这样一个情况：`can_fixed(mid)`为`True`，并且`mid`就是我们要找的最小时间。
# 如果你直接将`r`设置为`mid - 1`，你实际上已经排除了可能的正确答案。

# 在经典的二分搜索中，当我们找到一个满足条件的值，我们不会立即排除它，而是会继续搜索可能存在的更小的解。
# 这就是为什么我们将`r`设置为`mid`而不是`mid - 1`。
        
    
s = Solution()
ans = s.repairCars(ranks = [4,2,3,1], cars = 10)
print(ans)        
            
s = Solution()
ans = s.repairCars(ranks = [5,1,8], cars = 6)
print(ans)    

s = Solution()
ans = s.repairCars(ranks = [3,3,1,2,1,1,3,2,1], cars = 58)
print(ans)    