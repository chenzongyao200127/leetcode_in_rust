# 2454_下一个更大元素_IV
# https://leetcode.cn/problems/next-greater-element-iv/description/

from typing import List


class Solution:
    def secondGreaterElement(self, nums: List[int]) -> List[int]:
        res = [-1] * len(nums)
        s1 = []
        s2 = []

        for i, n in enumerate(nums):
            while s2 and n > nums[s2[-1]]:
                res[s2.pop()] = n

            j = len(s1) - 1
            while j >= 0 and n > nums[s1[j]]:
                j -= 1

            s2 += s1[j+1:]
            del s1[j + 1:]

            s1.append(i)

        return res


s = Solution()
ans = s.secondGreaterElement(nums=[2, 4, 0, 9, 6])
print(ans)
