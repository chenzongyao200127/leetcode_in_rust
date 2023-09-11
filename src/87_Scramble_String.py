# 87_Scramble_String
# https://leetcode.cn/problems/scramble-string/description/

# We can scramble a string s to get a string t using the following algorithm:

# 1. If the length of the string is 1, stop.
# 2. If the length of the string is > 1, do the following:
#   - Split the string into two non-empty substrings at a random index, i.e., if the string is s, 
#     divide it to x and y where s = x + y.
#   - Randomly decide to swap the two substrings or to keep them in the same order. i.e., 
#     after this step, s may become s = x + y or s = y + x.
#   - Apply step 1 recursively on each of the two substrings x and y.

# Given two strings s1 and s2 of the same length, 
# return true if s2 is a scrambled string of s1, otherwise, return false.

# 示例 1：
# 输入：s1 = "great", s2 = "rgeat"
# 输出：true
# 解释：s1 上可能发生的一种情形是：
# "great" --> "gr/eat" // 在一个随机下标处分割得到两个子字符串
# "gr/eat" --> "gr/eat" // 随机决定：「保持这两个子字符串的顺序不变」
# "gr/eat" --> "g/r / e/at" // 在子字符串上递归执行此算法。两个子字符串分别在随机下标处进行一轮分割
# "g/r / e/at" --> "r/g / e/at" // 随机决定：第一组「交换两个子字符串」，第二组「保持这两个子字符串的顺序不变」
# "r/g / e/at" --> "r/g / e/ a/t" // 继续递归执行此算法，将 "at" 分割得到 "a/t"
# "r/g / e/ a/t" --> "r/g / e/ a/t" // 随机决定：「保持这两个子字符串的顺序不变」
# 算法终止，结果字符串和 s2 相同，都是 "rgeat"
# 这是一种能够扰乱 s1 得到 s2 的情形，可以认为 s2 是 s1 的扰乱字符串，返回 true

# 示例 2：
# 输入：s1 = "abcde", s2 = "caebd"
# 输出：false

# 示例 3：
# 输入：s1 = "a", s2 = "a"
# 输出：true

class Solution:
    def isScramble(self, s1: str, s2: str) -> bool:
        