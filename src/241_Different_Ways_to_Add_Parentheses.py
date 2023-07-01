# 241_Different_Ways_to_Add_Parentheses
# https://leetcode.cn/problems/different-ways-to-add-parentheses/

from typing import List

# 分治
# 对于一个形如 x op y（op 为运算符，x 和 y 为数） 的算式而言，它的结果组合取决于 x 和 y 的结果组合数，而 x 和 y 又可以写成形如 x op y 的算式。
# 因此，该问题的子问题就是 x op y 中的 x 和 y：以运算符分隔的左右两侧算式解。

# 然后我们来进行 分治算法三步走：
#   分解：按运算符分成左右两部分，分别求解
#   解决：实现一个递归函数，输入算式，返回算式解
#   合并：根据运算符合并左右两部分的解，得出最终解
class Solution:
    def diffWaysToCompute(self, expression: str) -> List[int]:
        if expression.isdigit():
            return [int(expression)]
        
        memo = {}
        
        def Compute(expression, res):
            if expression.isdigit():
                return [int(expression)]
            
            for i in range(len(expression)):
                if expression[i] in {'+', '-', '*'}:
                    left_res = memo.get(expression[:i], Compute(expression[:i], []))
                    right_res = memo.get(expression[i+1:], Compute(expression[i+1:], []))
                    for left in left_res:
                        for right in right_res:
                            if expression[i] == '+':
                                res.append(left + right)
                            elif expression[i] == '-':
                                res.append(left - right)
                            elif expression[i] == '*':
                                res.append(left * right)
                    memo[expression[:i]] = left_res
                    memo[expression[i+1:]] = right_res
            return res
                    
        return Compute(expression, [])



# 回溯搜索。该问题牵涉到括号的组合问题，一般使用递归+回溯的思想。主要想法：
# 其一，递归回溯。可以产生所有的组合方式。
# 其二，每个小组合方式相当于一个子集，不断的将计算结果返回给上一层。
# 举例：a + (b - (c * d))会不断的变成a + (b - (res1 * res2)) -> a + (res1 - res2) -> res1 + res2
# 似乎计算结果不需要for循环？其实有这种情况，a + (b - (c * d))和a + (b - c) * d))，这里 a + res2，res2就可能有多种情况。
# 介绍一个API：substr第一个参数为起始位置，第二个参数为长度。如果不设置长度的话，默认为到最尾部。
