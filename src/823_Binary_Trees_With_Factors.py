# 823_Binary_Trees_With_Factors
# https://leetcode.cn/problems/binary-trees-with-factors/

from typing import List

class Solution:
    def numFactoredBinaryTrees(self, arr: List[int]) -> int:
        m = 10**9 + 7
        arr.sort()
        n = len(arr)
        dp = [1] * n
        for i in range(1, n):
            l = 0
            r = i-1
            tmp = arr[l] * arr[r]
            while l <= r:
                if tmp < arr[i]:
                    l += 1
                elif tmp > arr[i]:
                    r -= 1
                else:
                    if l == r:
                        dp[i] = (dp[i] + (dp[l] * dp[l])) % m
                    else:
                        dp[i] = (dp[i] + 2 * (dp[l] * dp[r])) % m
                    l += 1
                    r -= 1
                tmp = arr[l] * arr[r]

        return sum(dp) % m


# 优化
from typing import List

class Solution:
    def numFactoredBinaryTrees(self, arr: List[int]) -> int:
        MOD = 10**9 + 7
        arr.sort()
        
        # 用哈希表记录每个值的索引，以便快速查找
        index = {num: i for i, num in enumerate(arr)}
        
        dp = [1] * len(arr)
        for i, num in enumerate(arr):
            for j in range(i):
                if num % arr[j] == 0: # arr[j]是可能的左子节点
                    right_child = num // arr[j]
                    if right_child in index: # 如果也在arr中，那么它是一个有效的右子节点
                        dp[i] += dp[j] * dp[index[right_child]]
                        dp[i] %= MOD
        
        return sum(dp) % MOD


        
s = Solution()
ans = s.numFactoredBinaryTrees(arr = [2, 4])
print(ans) # 3


s = Solution()
ans = s.numFactoredBinaryTrees(arr = [2, 4, 5, 10])
print(ans) # 7

s = Solution()
ans = s.numFactoredBinaryTrees(arr = [18,3,6,2])
print(ans) # 12

s = Solution()
ans = s.numFactoredBinaryTrees(arr = [45,42,2,18,23,1170,12,41,40,9,47,24,33,28,10,32,29,17,46,11,759,37,6,26,21,49,31,14,19,8,13,7,27,22,3,36,34,38,39,30,43,15,4,16,35,25,20,44,5,48])
print(ans) # 777