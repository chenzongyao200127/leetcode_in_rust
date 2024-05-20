# 1177_构建回文串检测
# https://leetcode.cn/problems/can-make-palindrome-from-substring/description/

from typing import List


class Solution:
    def canMakePaliQueries(self, s: str, queries: List[List[int]]) -> List[bool]:
        sum = [[0] * 26]
        for c in s:
            sum.append(sum[-1].copy())
            sum[-1][ord(c) - ord('a')] += 1

        ans = []
        for left, right, k in queries:
            m = 0
            for sl, sr in zip(sum[left], sum[right + 1]):
                m += (sr - sl) % 2  # 奇数+1，偶数+0
            ans.append(m // 2 <= k)
        return ans


class Solution:
    def canMakePaliQueries(self, s: str, queries: List[List[int]]) -> List[bool]:
        sum = [[0] * 26]
        for c in s:
            sum.append(sum[-1].copy())
            sum[-1][ord(c) - ord('a')] += 1
            sum[-1][ord(c) - ord('a')] %= 2  # 偶数是 0

        ans = []
        for left, right, k in queries:
            m = 0
            for sl, sr in zip(sum[left], sum[right + 1]):
                m += sr != sl
            ans.append(m // 2 <= k)
        return ans


class Solution:
    def canMakePaliQueries(self, s: str, queries: List[List[int]]) -> List[bool]:
        sum = [0]
        for c in s:
            bit = 1 << (ord(c) - ord('a'))
            sum.append(sum[-1] ^ bit)  # 该比特对应字母的奇偶性：奇数变偶数，偶数变奇数

        ans = []
        for left, right, k in queries:
            m = (sum[left] ^ sum[right + 1]).bit_count()
            ans.append(m // 2 <= k)
        return ans
