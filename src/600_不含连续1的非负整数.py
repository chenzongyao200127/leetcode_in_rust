# 600_不含连续1的非负整数
# https://leetcode.cn/problems/non-negative-integers-without-consecutive-ones/description/

# 给定一个正整数 n ，请你统计在 [0, n] 范围的非负整数中，有多少个整数的二进制表示中不存在连续的 1 。

# 示例 1:

# 输入: n = 5
# 输出: 5
# 解释: 
# 下面列出范围在 [0, 5] 的非负整数与其对应的二进制表示：
# 0 : 0
# 1 : 1
# 2 : 10
# 3 : 11
# 4 : 100
# 5 : 101
# 其中，只有整数 3 违反规则（有两个连续的 1 ），其他 5 个满足规则。


# 示例 2:
# 输入: n = 1
# 输出: 2

# 示例 3:
# 输入: n = 2
# 输出: 3

# class Solution:
#     def findIntegers(self, n: int) -> int:
#         dp = [1] * 10000
    
#         for i in range(2, n+1):
#             low = i - (1 << (i.bit_length() - 1))
#             print(low)
#             if low >= (1 << (i.bit_length() - 1)) or dp[low] != 1:
#                 dp[i] = 0
        
#         print(dp[:n+1])
#         return sum(dp[:n+1])
    
# s = Solution()
# ans = s.findIntegers(3)
# print(ans)    


# 思考过程：

# 1. 一个整数的二进制表示中不能有连续的1。
# 2. 对于一个位长为`k`的整数，它的二进制表示中最后两位可以是`00`、`01`或`10`，但不能是`11`。
# 3. 设`f[k]`表示位长为`k`的整数，其二进制表示中不含连续`1`的数量。
# 
# 我们有以下转移方程：

# ```
# f[k] = f[k-1] + f[k-2]
# ```

# 为什么？考虑第`k`位：
# - 如果第`k`位是0，那么前`k-1`位可以是任意不含连续1的组合（总共`f[k-1]`种）。
# - 如果第`k`位是1，那么第`k-1`位必须是0，这样前`k-2`位可以是任意不含连续1的组合（总共`f[k-2]`种）。

# 考虑到上述递推关系，我们可以使用动态规划来解决这个问题。

# 重新编写代码：


class Solution:
    def findIntegers(self, n: int) -> int:
        bin_n = bin(n)[2:]
        k = len(bin_n)
        
        # 初始化 dp 数组
        dp = [0] * (k + 1)
        dp[0] = 1
        dp[1] = 2
        
        # 根据递推关系填充 dp 数组
        for i in range(2, k):
            dp[i] = dp[i-1] + dp[i-2]

        ans, prev_bit = 0, 0
        
        for i in range(k):
            curr_bit = int(bin_n[i])
            if curr_bit == 1:
                ans += dp[k-i-1]
                if prev_bit == 1:
                    return ans
            prev_bit = curr_bit

        return ans + 1

s = Solution()
print(s.findIntegers(3))  # 输出3

# 这部分代码是为了计算小于等于`n`并且其二进制表示中不包含连续1的数字的数量。

# 1. `ans`：初始化为0，代表我们计算的满足条件的数的数量。
# 2. `prev_bit`：表示上一次循环中处理的`n`的二进制位。

# 我们从`n`的最高位开始检查每一个二进制位：

# 1. `curr_bit = int(bin_n[i])`: 获取当前处理的二进制位。

# 2. 如果`curr_bit`为1：
#    - `ans += dp[k-i-1]`: 加上所有小于当前位的满足条件的数的数量。
#       例如，对于`n = 1011`，当我们检查第二位(0-based)时，这一位是1，
#       所以我们可以计算所有满足条件的小于`1000`的数。
#    - `if prev_bit == 1`: 如果前一个位也是1，那么我们已经找到了连续的1，所以直接返回当前的`ans`，因为更低的位数不再重要。

# 3. `prev_bit = curr_bit`: 更新`prev_bit`为当前的位，以备下次循环使用。

# 最后，循环结束后，我们返回`ans + 1`。为什么要加1？因为`n`本身也是一个满足条件的数。

class Solution:
    def findIntegers(self, n: int) -> int:
        dp = [0] * 31
        dp[0] = 1
        dp[1] = 1
        for i in range(2, 31):
            dp[i] = dp[i - 1] + dp[i - 2]

        pre = 0
        res = 0

        for i in range(29, -1, -1):
            val = (1 << i)
            if n & val:
                res += dp[i + 1]
                if pre == 1:
                    break
                pre = 1
            else:
                pre = 0

            if i == 0:
                res += 1

        return res
