# 1657. 确定两个字符串是否接近
# https://leetcode.cn/problems/determine-if-two-strings-are-close/description/

from collections import Counter

class Solution:
    def closeStrings(self, s: str, t: str) -> bool:
        if len(s) != len(t):
            return False
        cs, ct = Counter(s), Counter(t)
        return cs.keys() == ct.keys() and Counter(cs.values()) == Counter(ct.values())
    