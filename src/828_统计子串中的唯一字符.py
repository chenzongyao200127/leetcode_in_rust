# 828_统计子串中的唯一字符
# https://leetcode.cn/problems/count-unique-characters-of-all-substrings-of-a-given-string/description/

# 我们定义了一个函数 `countUniqueChars(s)` 来统计字符串 s 中的唯一字符，并返回唯一字符的个数。

# 例如：s = "LEETCODE" ，则其中 "L", "T","C","O","D" 都是唯一字符，因为它们只出现一次，所以 countUniqueChars(s) = 5 。

# 本题将会给你一个字符串 s ，我们需要返回 countUniqueChars(t) 的总和，其中 t 是 s 的子字符串。输入用例保证返回值为 32 位整数。

# 注意，某些子字符串可能是重复的，但你统计时也必须算上这些重复的子字符串（也就是说，你必须统计 s 的所有子字符串中的唯一字符）。


# 您提供的示例说明了问题的核心：计算由独特字符构成的所有子字符串的长度之和。

# 这个问题可以通过动态规划或者使用特定的数据结构（如前面提到的线段树）来解决。
# 然而，有一种更高效的方法可以用来解决这个问题，它涉及到跟踪每个字符上一次出现的位置。

# 思路如下：
# 对于字符串中的每个字符，跟踪该字符最近一次出现的位置。
# 当处理到字符串的第 i 个字符时，计算以该字符结尾的独特子字符串的数量。这可以通过比较当前位置 i 和该字符上一次出现的位置来实现。
# 累加每个位置得到的独特子字符串数量，得到最终结果。

# 我将根据这个思路为您编写代码。
class Solution:
    def uniqueLetterString(self, s: str) -> int:
        # 记录每个字符最后一次出现的位置
        last_position = [-1] * 26
        # 记录每个字符上一次贡献的子串长度
        contribution = [0] * 26
        total = 0
        curr_sum = 0

        for i, char in enumerate(s):
            idx = ord(char) - ord('A')
            # 计算当前字符新贡献的子串长度（当前位置减去该字符上次出现的位置）
            curr_sum -= contribution[idx]
            # 更新当前字符的贡献
            contribution[idx] = i - last_position[idx]
            curr_sum += contribution[idx]
            # 更新总和
            total += curr_sum
            # 更新该字符最后出现的位置
            last_position[idx] = i

        return total
