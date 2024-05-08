# 1652_拆炸弹
# https://leetcode.cn/problems/defuse-the-bomb/description/?envType=daily-question&envId=2024-05-05


from typing import List


class Solution:
    def decrypt(self, code: List[int], k: int) -> List[int]:
        n = len(code)
        if k == 0:
            return [0] * n
        if k > 0:
            code += code
            for i in range(n):
                code[i] = sum(code[i + 1:i + k + 1])
            return code[:n]
        else:
            code = code[::-1] + code[::-1]
            for i in range(n):
                code[i] = sum(code[i + 1:i - k + 1])
            res = code[:n]
            return res[::-1]
