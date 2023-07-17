# 415_Add_Strings
# https://leetcode.cn/problems/add-strings/

# 给定两个字符串形式的非负整数 num1 和num2 ，计算它们的和并同样以字符串形式返回。

# 你不能使用任何內建的用于处理大整数的库（比如 BigInteger）， 也不能直接将输入的字符串转换为整数形式。
# 示例 1：
# 输入：num1 = "11", num2 = "123"
# 输出："134"

# 示例 2：
# 输入：num1 = "456", num2 = "77"
# 输出："533"

# 示例 3：
# 输入：num1 = "0", num2 = "0"
# 输出："0"

class Solution:
    def addStrings(self, num1: str, num2: str) -> str:
        num1 = list(num1)
        num2 = list(num2)
        num1.reverse()
        num2.reverse()
        n1 = len(num1)
        n2 = len(num2)
        n_min = min(n1, n2)
        n_max = max(n1, n2)
        if n1 < n2:
            num1, num2 = num2, num1
        ca = 0
        ans = []
        for i in range(n_max):
            a = int(num1[i])
            if i >= n_min:
                b = 0
            else:
                b = int(num2[i])
            tmp = a + b + ca
            ans.append(str(tmp % 10))
            ca = tmp // 10
        if ca:
            ans.append(str(ca))
        ans.reverse()
        return "".join(ans)
    
    
# 代码优化
class Solution:
    def addStrings(self, num1: str, num2: str) -> str:
        res = ""
        i, j, carry = len(num1) - 1, len(num2) - 1, 0
        while i >= 0 or j >= 0:
            n1 = int(num1[i]) if i >= 0 else 0
            n2 = int(num2[j]) if j >= 0 else 0
            tmp = n1 + n2 + carry
            carry = tmp // 10
            res = str(tmp % 10) + res
            i, j = i - 1, j - 1
        return "1" + res if carry else res