# 22_Generate_Parentheses
# https://leetcode.cn/problems/generate-parentheses/

# 数字 n 代表生成括号的对数，请你设计一个函数，用于能够生成所有可能的并且 有效的 括号组合。
# 示例 1：
# 输入：n = 3
# 输出：["((()))","(()())","(())()","()(())","()()()"]
# 示例 2：
# 输入：n = 1
# 输出：["()"]

from typing import List
from itertools import permutations

# 这段代码尝试生成n对括号的所有有效组合。
# 虽然你的验证方法检查了生成的括号字符串是否有效，但是生成所有可能的组合的方法有一些问题。
# 在这里，你首先生成了n对括号，然后试图找出这些括号的所有排列。
# 然而，这并不能保证生成的字符串都是有效的，而且会产生大量的重复项。
# 这是因为permutations函数不会区分相同的项，例如它会将两个左括号看作是不同的。
def generateParenthesis(n: int) -> List[str]:
    ans = []
    parenthesis = ['(', ')'] * n
    def is_valid(array):
        stack = []
        for x in array:
            stack.append(x)
            if len(stack) >= 2:
                if stack[-1] == ')' and stack[-2] == '(':
                    stack.pop()
                    stack.pop()
        return stack == []
    
    for arrange in permutations(parenthesis):
        array = []
        for parent in arrange:
            array.append(parent)
        if is_valid(array):
            ans.append("".join(array))
    
    return ans
        
# 这个函数生成 n 对括号的所有有效组合。
# 递归调用 generate 函数，当我们有左括号时添加一个左括号，
# 当我们有更多的右括号时添加一个右括号，当我们没有右括号时，
# 我们知道已经完成一个有效的括号组合，所以将其加入结果列表中。
def generateParenthesis(n: int) -> List[str]:
    def generate(p: List[str], left: int, right: int, parens: List[str] = []):
        if left: generate(p + ['('], left - 1, right)
        if right > left: generate(p + [')'], left, right - 1)
        if not right: parens.append(''.join(p))
        return parens

    return generate([], n, n)
    
print(generateParenthesis(3))


def generateParenthesis(n: int) -> List[str]:
    def backtrack(s = '', left = 0, right = 0):
        if len(s) == 2 * n:
            ans.append(s)
            return
        if left < n:
            backtrack(s+'(', left+1, right)
        if right < left:
            backtrack(s+')', left, right+1)

    ans = []
    backtrack()
    return ans
