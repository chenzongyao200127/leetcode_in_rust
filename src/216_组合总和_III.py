# 216_组合总和_III
# https://leetcode.cn/problems/combination-sum-iii/description/

from typing import List


class Solution:
    def combinationSum3(self, k: int, n: int) -> List[List[int]]:

        def backtrack(start, remaining, path):
            if remaining == 0 and len(path) == k:
                result.append(path.copy())
                return
            elif len(path) == k:
                return

            for i in range(start, 10):
                if i > remaining:
                    break
                path.append(i)
                backtrack(i + 1, remaining - i, path)
                path.pop()

        result = []
        backtrack(1, n, [])
        return result
