# 282_Expression_Add_Operators
# https://leetcode.cn/problems/expression-add-operators/

from typing import List

class Solution:
    def addOperators(self, num: str, target: int) -> List[str]:
        n = len(num)  # 获取输入字符串的长度
        ans = []  # 初始化结果列表

        # 定义回溯函数
        # expr: 当前构建的表达式
        # i: 当前处理的数字的索引
        # res: 当前表达式的计算结果
        # mul: 最近一次乘法操作的值（用于处理乘法优先级）⭐
        def backtrack(expr: List[str], i: int, res: int, mul: int):
            if i == n:  # 如果处理完所有数字
                if res == target:  # 如果当前表达式的计算结果等于目标值
                    ans.append(''.join(expr))  # 将表达式添加到结果列表中
                return
            
            signIndex = len(expr)  # 记录当前符号的索引位置
            
            if i > 0:
                expr.append('')  # 占位，稍后填充符号
                
            val = 0  # 初始化当前处理的数字的值
            for j in range(i, n):  # 枚举截取的数字长度（取多少位）
                if j > i and num[i] == '0':  # 数字可以是单个 0 但不能有前导零
                    break
                
                val = val * 10 + int(num[j])  # 计算当前处理的数字的值
                expr.append(num[j])  # 将当前处理的数字添加到表达式中
                
                if i == 0:  # 如果是表达式的开头，不能添加符号
                    backtrack(expr, j + 1, val, val)  # 继续处理下一个数字
                
                else:  # 如果不是表达式的开头，需要枚举符号
                    expr[signIndex] = '+'; backtrack(expr, j + 1, res + val, val)
                    expr[signIndex] = '-'; backtrack(expr, j + 1, res - val, -val)
                    expr[signIndex] = '*'; backtrack(expr, j + 1, res - mul + mul * val, mul * val)
                    
            del expr[signIndex:]  # 回溯时删除已加入的符号和数字

        backtrack([], 0, 0, 0)  # 从第一个数字开始回溯
        return ans  # 返回结果列表
    
    
    
from typing import List

# NB 回溯 + 剪枝 优化
class Solution:
    def addOperators(self, num: str, target: int) -> List[str]:
        ans = []  # 初始化结果列表

        # 定义深度优先搜索函数
        # left: 当前处理的数字的索引
        # prev_num: 前一个数字（用于处理乘法优先级）
        # cur_ans: 当前表达式的计算结果
        # expression: 当前构建的表达式
        def dfs(left: int, prev_num: int, cur_ans: int, expression: str):
            if left == len(num):  # 如果处理完所有数字
                if cur_ans == target:  # 如果当前表达式的计算结果等于目标值
                    ans.append(expression)  # 将表达式添加到结果列表中
                return
            
            for i in range(left, len(num)):
                if i != left and num[left] == "0":  # 数字可以是单个 0 但不能有前导零
                    return
                
                cur_num = int(num[left:i + 1])  # 计算当前处理的数字的值
                
                if expression == "":  # 如果是表达式的开头，不能添加符号
                    dfs(i + 1, cur_num, cur_num, num[left:i + 1])
                    continue

                # 剪枝，若num[left:]的值小于与目标的差值，则只能进行乘法计算
                if int(num[left:]) >= abs(target - cur_ans):
                    # 相加
                    dfs(i + 1, cur_num, cur_ans + cur_num, expression + "+" + num[left:i + 1])
                    # 相减
                    dfs(i + 1, -cur_num, cur_ans - cur_num, expression + "-" + num[left:i + 1])

                # 相乘
                dfs(i + 1, prev_num * cur_num, cur_ans - prev_num + prev_num * cur_num, expression + "*" + num[left:i + 1])

        dfs(0, 0, 0, "")  # 从第一个数字开始深度优先搜索

        return ans  # 返回结果列表