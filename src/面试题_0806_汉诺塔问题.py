# 面试题_0806_汉诺塔问题

from typing import List

# 使用迭代实现

# 在经典汉诺塔问题中，有 3 根柱子及 N 个不同大小的穿孔圆盘，盘子可以滑入任意一根柱子。
# 一开始，所有盘子自上而下按升序依次套在第一根柱子上(即每一个盘子只能放在更大的盘子上面)。移动圆盘时受到以下限制:
# (1) 每次只能移动一个盘子;
# (2) 盘子只能从柱子顶端滑出移到下一根柱子;
# (3) 盘子只能叠在比它大的盘子上。
# 请编写程序，用栈将所有盘子从第一根柱子移到最后一根柱子。
# 你需要原地修改栈。


class Solution:
    def hanota(self, A: List[int], B: List[int], C: List[int]) -> None:
        n = len(A)
        self.move(n, A, B, C)

    # 定义move 函数移动汉诺塔
    def move(self, n, A, B, C):
        if n == 1:
            C.append(A[-1])
            A.pop()
            return
        else:
            self.move(n-1, A, C, B)  # 将A上面n-1个通过C移到B
            C.append(A[-1])          # 将A最后一个移到C
            A.pop()                  # 这时，A空了
            self.move(n-1, B, A, C)  # 将B上面n-1个通过空的A移到C

# 万能的递归转迭代写法（模拟栈）


class Solution:
    def hanota(self, A: List[int], B: List[int], C: List[int]) -> None:
        n = len(A)
        stack = [(n, A, C, B)]  # (盘子数量，源柱子，目标柱子，辅助柱子)
        while stack:
            num, src, dest, aux = stack.pop()
            if num == 1:
                dest.append(src.pop())
            else:
                # 逆序推入栈中以保证执行顺序正确
                # 将 n-1 个盘子从辅助移到目标，源做辅助
                stack.append((num - 1, aux, dest, src))
                stack.append((1, src, dest, aux))        # 将最大盘子从源移到目标
                # 将 n-1 个盘子从源移到辅助，目标做辅助
                stack.append((num - 1, src, aux, dest))


# 将递归转换为迭代的通用方法确实涉及使用栈来模拟递归调用栈的操作。
# 这种方法可以应用于各种递归问题，包括二叉树遍历、图的深度优先搜索、分治算法等。
# 下面，我将详细提供一个通用的递归转迭代方法，以及如何将这种方法应用到汉诺塔问题上。

# 递归转迭代的通用方法
# 定义一个栈：栈用于保存每一步的状态，模拟递归调用的过程。
# 初始化栈：将递归函数的初始参数放入栈中。
# 处理栈：
# 循环直到栈为空。
# 弹出栈顶元素，这代表当前的“递归调用”。
# 根据当前的状态和需要解决的问题，可能需要将更多的状态推回栈中，代表未来的递归调用。
