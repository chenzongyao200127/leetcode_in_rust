# // 402_Remove_K_Digits
# // https://leetcode.cn/problems/remove-k-digits/
    
    
# 思路，从左到右，找第一个比后面大的字符，删除，清零，k次扫描。    
class Solution:
    def removeKdigits(self, num: str, k: int) -> str:
        stk = []
        for c in num:
            while stk and c < stk[-1] and k:
                stk.pop()
                k -= 1
            stk.append(c)
        res = ''.join(stk)[:len(stk) - k].lstrip('0')
        return res if res else '0'