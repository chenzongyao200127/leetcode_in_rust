# 371_Sum_of_Two_Integers
# https://leetcode.cn/problems/sum-of-two-integers/description/

class Solution:
    def getSum(self, a: int, b: int) -> int:
        binary_a_representation = bin(a)
        binary_b_representation = bin(b)
        print(binary_a_representation)
        print(binary_b_representation)


s = Solution()
print(s.getSum(22, -22))