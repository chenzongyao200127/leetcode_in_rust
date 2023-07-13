# 338_Counting_Bits
# https://leetcode.cn/problems/counting-bits/

# 给你一个整数 n ，对于 0 <= i <= n 中的每个 i ，计算其二进制表示中 1 的个数 ，返回一个长度为 n + 1 的数组 ans 作为答案。

# 示例 1：
# 输入：n = 2
# 输出：[0,1,1]
# 解释：
# 0 --> 0
# 1 --> 1
# 2 --> 10

# 示例 2：
# 输入：n = 5
# 输出：[0,1,1,2,1,2]
# 解释：
# 0 --> 0
# 1 --> 1
# 2 --> 10
# 3 --> 11
# 4 --> 100
# 5 --> 101

# You should make use of what you have produced already.
# Divide the numbers in ranges like [2-3], [4-7], [8-15] and so on. And try to generate new range from previous.
# Or does the odd/even status of the number help you in calculating the number of 1s?

# 0 <= n <= 105
from typing import List

# DP
class Solution:
    def countBits(self, n: int) -> List[int]:
        i = 0
        ans = [0] * (n+1)
        for i in range(n+1):
            if i % 2 == 0:
                ans[i] = ans[i//2]
            else:
                ans[i] = ans[i-1] + 1

        return ans

# 这道题需要计算从 0 到 n 的每个整数的二进制表示中的 1 的数目。

# 部分编程语言有相应的内置函数用于计算给定的整数的二进制表示中的 1 的数目，读者可以自行尝试。下列各种方法均为不使用内置函数的解法。

# 为了表述简洁，下文用「一比特数」表示二进制表示中的 1 的数目。

# 方法一：Brian Kernighan 算法

class Solution:
    def countBits(self, n: int) -> List[int]:
        def countOnes(x: int) -> int:
            ones = 0
            while x > 0:
                x &= (x - 1)
                ones += 1
            return ones
        
        bits = [countOnes(i) for i in range(n + 1)]
        return bits

# Brian Kernighan 算法（也称为 BK 算法或 Brian Kernighan's Algorithm）是一种用于计算二进制数中位数的算法。
# 它的主要思想是使用位运算来找出一个整数中 1 的个数，从而计算出二进制数中的位数。

# 算法步骤如下：

#   初始化计数器 count 为 0。
#   当输入的整数 n 不为 0 时，执行以下步骤：
#       令 n = n & (n - 1)，即将 n 的最右边的 1 变为 0，并将结果赋给 n。
#       计数器 count 加 1。
#   返回计数器 count。
#   使用位运算 & 和减法 -，可以将 n 的最右边的 1 变为 0。
#   例如，当 n = 101100 时，执行 n = n & (n - 1) 后，得到 n = 101000。这是因为 n - 1 的二进制数中，
#   原来的最右边的 1 变为了 0，而它右边的所有位都变为了 1，因此与 n 进行按位与运算后，原来的最右边的 1 变为了 0。
    
# 由于每次执行 n = n & (n - 1) 后，都会将 n 中的一个 1 变为 0，因此最终 n 变为 0 时，计数器 count 中的值就是二进制数中 1 的个数。
# Brian Kernighan 算法的时间复杂度为 O(k)，其中 k 是二进制数中 1 的个数。
# 由于 k 最大为二进制数的位数，因此 Brian Kernighan 算法的时间复杂度为 O(log n)。
# 该算法的空间复杂度为 O(1)，因为它只需要一个计数器变量。
