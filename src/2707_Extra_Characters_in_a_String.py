# 2707_Extra_Characters_in_a_String
# https://leetcode.cn/problems/extra-characters-in-a-string/description/

from typing import List


class Solution:
    def minExtraChar(self, s: str, dictionary: List[str]) -> int:
        d = set(dictionary)

        @cache
        def dfs(i):
            if i < 0:
                return 0
            res = dfs(i-1) + 1
            for j in range(i+1):
                if s[j:i+1] in d:
                    res = min(res, dfs(j-1))
            return res

        return dfs(len(s)-1)


class Solution:
    def minExtraChar(self, s: str, dictionary: List[str]) -> int:
        d = set(dictionary)
        n = len(s)
        f = [0] * (n + 1)
        for i in range(n):
            f[i + 1] = f[i] + 1

            for j in range(i + 1):
                if s[j:i + 1] in d:
                    f[i + 1] = min(f[i + 1], f[j])

        return f[n]
