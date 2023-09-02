# 2511_Maximum_Enemy_Forts_That_Can_Be_Captured
# https://leetcode.cn/problems/maximum-enemy-forts-that-can-be-captured/

# Example 1:
# Input: forts = [1,0,0,-1,0,0,0,0,1]
# Output: 4
# Explanation:
# - Moving the army from position 0 to position 3 captures 2 enemy forts, at 1 and 2.
# - Moving the army from position 8 to position 3 captures 4 enemy forts.
# Since 4 is the maximum number of enemy forts that can be captured, we return 4.

# Example 2:
# Input: forts = [0,0,1,-1]
# Output: 0
# Explanation: Since no enemy fort can be captured, 0 is returned.

from typing import List

class Solution:
    def captureForts(self, forts: List[int]) -> int:
        ans = 0
        for i in range(len(forts)):
            if forts[i] == 1:
                l, r = i, i
                
                while l > 0:
                    l -= 1
                    if forts[l] == -1:
                        ans = max(ans, i - l - 1)
                        break
                    if forts[l] == 1:
                        break
                
                while r < len(forts) - 1:
                    r += 1
                    if forts[r] == -1:
                        ans = max(ans, r - i - 1)
                        break
                    if forts[r] == 1:
                        break        
        
        return ans
    

s = Solution()
ans = s.captureForts([1,0,0,-1,0,0,0,0,1])
print(ans)
                    
s = Solution()
ans = s.captureForts([0,0,1,-1])
print(ans)

s = Solution()
ans = s.captureForts([1,-1,-1,1,1])
print(ans)

s = Solution()
ans = s.captureForts([1,0,0,-1,0,1])
print(ans)