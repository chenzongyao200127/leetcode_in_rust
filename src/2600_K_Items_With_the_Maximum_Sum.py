# 2600_K_Items_With_the_Maximum_Sum
# https://leetcode.cn/problems/k-items-with-the-maximum-sum/

class Solution:
    def kItemsWithMaximumSum(self, numOnes: int, numZeros: int, numNegOnes: int, k: int) -> int:
        if 0 <= k <= numOnes:
            return k
        elif numOnes <= k <= numOnes + numZeros:
            return numOnes
        else:
            return numOnes - (k - numOnes - numZeros)