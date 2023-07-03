# 679_24_Game
# https://leetcode.cn/problems/24-game/

# 给定一个长度为4的整数数组 cards 。你有 4 张卡片，每张卡片上都包含一个范围在 [1,9] 的数字。
# 您应该使用运算符 ['+', '-', '*', '/'] 和括号 '(' 和 ')' 将这些卡片上的数字排列成数学表达式，以获得值24。

# 你须遵守以下规则:

# 除法运算符 '/' 表示实数除法，而不是整数除法。
    # 例如， 4 /(1 - 2 / 3)= 4 /(1 / 3)= 12 。
# 每个运算都在两个数字之间。特别是，不能使用 “-” 作为一元运算符。
    # 例如，如果 cards =[1,1,1,1] ，则表达式 “-1 -1 -1 -1” 是 不允许 的。
# 你不能把数字串在一起
    # 例如，如果 cards =[1,2,1,2] ，则表达式 “12 + 12” 无效。
# 如果可以得到这样的表达式，其计算结果为 24 ，则返回 true ，否则返回 false 。

# 示例 1:
# 输入: cards = [4, 1, 8, 7]
# 输出: true
# 解释: (8-4) * (7-1) = 24

# 示例 2:
# 输入: cards = [1, 2, 1, 2]
# 输出: false
from typing import List


# GPT - 4
from itertools import permutations, product
from operator import add, sub, mul, truediv

def judgePoint24(cards: List[int]) -> bool:
    operators = [add, sub, mul, truediv]
    for nums in permutations(cards):
        print(nums)
        for ops in product(operators, repeat=3):
            try:
                if abs(ops[0](ops[1](nums[0], nums[1]), ops[2](nums[2], nums[3])) - 24) < 1e-6:
                    return True
                if abs(ops[0](nums[0], ops[1](ops[2](nums[1], nums[2]), nums[3])) - 24) < 1e-6:
                    return True
                if abs(ops[0](nums[0], ops[1](nums[1], ops[2](nums[2], nums[3]))) - 24) < 1e-6:
                    return True
            except ZeroDivisionError:
                continue
    return False

print(judgePoint24([4,1,8,7]))

# itertools.product(*iterables, repeat=1)
# 可迭代对象输入的笛卡儿积。
# 大致相当于生成器表达式中的嵌套循环。例如， product(A, B) 和 ((x,y) for x in A for y in B) 返回结果一样。

# 嵌套循环像里程表那样循环变动，每次迭代时将最右侧的元素向后迭代。
# 这种模式形成了一种字典序，因此如果输入的可迭代对象是已排序的，笛卡尔积元组依次序发出。

# 要计算可迭代对象自身的笛卡尔积，将可选参数 repeat 设定为要重复的次数。
# 例如，product(A, repeat=4) 和 product(A, A, A, A) 是一样的。

# 该函数大致相当于下面的代码，只不过实际实现方案不会在内存中创建中间结果。
def product(*args, repeat=1):
    # product('ABCD', 'xy') --> Ax Ay Bx By Cx Cy Dx Dy
    # product(range(2), repeat=3) --> 000 001 010 011 100 101 110 111
    pools = [tuple(pool) for pool in args] * repeat
    result = [[]]
    for pool in pools:
        result = [x+[y] for x in result for y in pool]
    for prod in result:
        yield tuple(prod)
# 在 product() 运行之前，它会完全耗尽输入的可迭代对象，在内存中保留值的临时池以生成结果积。 相应地，它只适用于有限的输入。



# itertools.permutations(iterable, r=None)
# 连续返回由 iterable 元素生成长度为 r 的排列。
# 如果 r 未指定或为 None ，r 默认设置为 iterable 的长度，这种情况下，生成所有全长排列。
def permutations(iterable, r=None):
    # permutations('ABCD', 2) --> AB AC AD BA BC BD CA CB CD DA DB DC
    # permutations(range(3)) --> 012 021 102 120 201 210
    pool = tuple(iterable)
    n = len(pool)
    r = n if r is None else r
    if r > n:
        return
    indices = list(range(n))
    cycles = list(range(n, n-r, -1))
    yield tuple(pool[i] for i in indices[:r])
    while n:
        for i in reversed(range(r)):
            cycles[i] -= 1
            if cycles[i] == 0:
                indices[i:] = indices[i+1:] + indices[i:i+1]
                cycles[i] = n - i
            else:
                j = cycles[i]
                indices[i], indices[-j] = indices[-j], indices[i]
                yield tuple(pool[i] for i in indices[:r])
                break
        else:
            return

# permutations() 的代码也可被改写为 product() 的子序列，只要将含有重复元素（来自输入中同一位置的）的项排除。
def permutations(iterable, r=None):
    pool = tuple(iterable)
    n = len(pool)
    r = n if r is None else r
    for indices in product(range(n), repeat=r):
        if len(set(indices)) == r:
            yield tuple(pool[i] for i in indices)
# 当 0 <= r <= n ，返回项个数为 n! / (n-r)! ；当 r > n ，返回项个数为0。
