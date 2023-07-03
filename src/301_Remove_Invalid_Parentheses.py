# 301_Remove_Invalid_Parentheses
# https://leetcode.cn/problems/remove-invalid-parentheses/

# 给你一个由若干括号和字母组成的字符串 s ，删除最小数量的无效括号，使得输入的字符串有效。
# 返回所有可能的结果。答案可以按 任意顺序 返回。


# 示例 1：
# 输入：s = "()())()"
# 输出：["(())()","()()()"]

# 示例 2：
# 输入：s = "(a)())()"
# 输出：["(a())()","(a)()()"]

# 示例 3：
# 输入：s = ")("
# 输出：[""]

from typing import List

class Solution:
    def removeInvalidParentheses(self, s: str) -> List[str]:
        