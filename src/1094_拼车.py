# 1094_拼车
# https://leetcode.cn/problems/car-pooling/description/

from typing import List

class Solution:
    def carPooling(self, trips: List[List[int]], capacity: int) -> bool:
        bus = [0] * 1001
        for trip in trips:
            numPassengersi, fromi, toi = trip
            for i in range(fromi, toi):
                bus[i] += numPassengersi
                print(bus[i])
                if bus[i] > capacity:
                    return False
            # print(numPassengersi, fromi, toi)
        return True
            
            
s = Solution()
ans = s.carPooling([[2,1,5],[3,3,7]],4)
print(ans)            