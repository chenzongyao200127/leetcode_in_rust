# 488_Zuma_Game
# https://leetcode.cn/problems/zuma-game/

# 你正在参与祖玛游戏的一个变种。

# 在这个祖玛游戏变体中，桌面上有 一排 彩球，每个球的颜色可能是：红色 'R'、黄色 'Y'、蓝色 'B'、绿色 'G' 或白色 'W' 。
# 你的手中也有一些彩球。

# 你的目标是 清空 桌面上所有的球。每一回合：

# 从你手上的彩球中选出 任意一颗 ，然后将其插入桌面上那一排球中：两球之间或这一排球的任一端。

# 接着，如果有出现 三个或者三个以上 且 颜色相同 的球相连的话，就把它们移除掉。

# 如果这种移除操作同样导致出现三个或者三个以上且颜色相同的球相连，则可以继续移除这些球，直到不再满足移除条件。

# 如果桌面上所有球都被移除，则认为你赢得本场游戏。

# 重复这个过程，直到你赢了游戏或者手中没有更多的球。

# 给你一个字符串 board ，表示桌面上最开始的那排球。另给你一个字符串 hand ，表示手里的彩球。
# 请你按上述操作步骤移除掉桌上所有球，计算并返回所需的 最少 球数。如果不能移除桌上所有的球，返回 -1 。


# 题解
# 方法一：广度优先搜索
# 根据题目要求，桌面上最多有 16 个球，手中最多有 5 个球；我们可以以任意顺序在 5 个回合中使用手中的球；
# 在每个回合中，我们可以选择将手中的球插入到桌面上任意两球之间或这一排球的任意一端。

# 因为插入球的颜色和位置的选择是多样的，选择的影响也可能在多次消除操作之后才能体现出来，所以通过贪心方法根据当前情况很难做出全局最优的决策。
# 实际每次插入一个新的小球时，并不保证插入后一定可以消除，因此我们需要搜索和遍历所有可能的插入方法，找到最小的插入次数。
# 比如以下测试用例：

# 桌面上的球为 RRWWRRBBRR，手中的球为 WB，如果我们按照贪心法每次插入进行消除就会出现无法完全消除。\
    
# 因此，我们使用广度优先搜索来解决这道题。即对状态空间进行枚举，通过穷尽所有的可能来找到最优解，并使用剪枝的方法来优化搜索过程。
# 为什么使用广度优先搜索？

# 我们不妨规定，每一种不同的桌面上球的情况和手中球的情况的组合都是一种不同的状态。
# 对于相同的状态，其清空桌面上球所需的回合数总是相同的；而不同的插入球的顺序，也可能得到相同的状态。
# 因此，如果使用深度优先搜索，则需要使用记忆化搜索，以避免重复计算相同的状态。

# 因为只需要找出需要回合数最少的方案，因此使用广度优先搜索可以得到可以消除桌面上所有球的方案时就直接返回结果，
# 而不需要继续遍历更多需要回合数更多的方案。
# 而广度优先搜索虽然需要在队列中存储较多的状态，但是因为使用深度优先搜索也需要存储这些状态及这些状态对应的结果，
# 因此使用广度优先搜索并不会需要更多的空间。

# 算法

# 在算法的实现中，我们可以通过以下方法来实现广度优先：

# 使用队列来维护需要处理的状态队列，使用哈希集合存储已经访问过的状态。
# 每一次取出队列中的队头状态，考虑其中所有可以插入球的方案，如果新方案还没有被访问过，则将新方案添加到队列的队尾。

# 下面，我们考虑剪枝条件：

# 第 1 个剪枝条件：手中颜色相同的球每次选择时只需要考虑其中一个即可
# 如果手中有颜色相同的球，那么插入这些球中的哪一个都没有区别。
# 因此，手中颜色相同的球，我们只需要考虑其中一个即可。
# 在具体的实现中，我们可以先将手中的球排序，如果当前遍历的球的颜色和上一个遍历的球的颜色相同，则跳过当前遍历的球。

# 第 2 个剪枝条件：只在连续相同颜色的球的开头位置或者结尾位置插入新的颜色相同的球
# 如果桌面上有一个红球，那么在其左侧和右侧插入一个新的红球没有区别；
# 同理，如果桌面上有 2 个连续的红球，那么在其左侧、中间和右侧插入一个新的红球没有区别。
# 因此，如果新插入的球和桌面上某组连续颜色相同的球（也可以是 1 个）的颜色相同，我们只需要考虑在其左侧插入新球的情况即可。
# 在具体的实现中，如果新插入的球和插入位置左侧的球的颜色相同，则跳过这个位置。

# 第 3 个剪枝条件：只考虑放置新球后有可能得到更优解的位置
# 考虑插入新球的颜色与插入位置周围球的颜色的情况，在已经根据第 2 个剪枝条件剪枝后，
# 还可能出现如下三种情况：
#   插入新球与插入位置右侧的球颜色相同；
#   插入新球与插入位置两侧的球颜色均不相同，且插入位置两侧的球的颜色不同；
#   插入新球与插入位置两侧的球颜色均不相同，且插入位置两侧的球的颜色相同。
# 对于「插入新球与插入位置右侧的球颜色相同」的情况，这种操作可能可以构成连续三个相同颜色的球实现消除，是有可能得到更优解的。
# 读者可以结合以下例子理解。
# 例如：桌面上的球为 WWRRBBWW，手中的球为 WWRB，答案为 2。
# 操作方法如下：WWRRBBWW→WW(R)RRBBWW→WWBBWW→WW(B)BBWW→WWWW→""
# 对于「插入新球与插入位置两侧的球颜色均不相同，且插入位置两侧的球的颜色不同」的情况，
# 这种操作可以将连续相同颜色的球拆分到不同的组合中消除，也是有可能得到更优解的。读者可以结合以下例子理解。
# 例如：桌面上的球为 RRWWRRBBRR，手中的球为 WB，答案为 2。
# 操作方法如下：RRWWRRBBRR→RRWWRRBBR(W)R→RRWWRR(B)BBRWR→RRWWRRRWR→RRWWWR→RRR→""
# 对于「插入新球与插入位置两侧的球颜色均不相同，且插入位置两侧的球的颜色相同」的情况，这种操作并不能对消除顺序产生任何影响。
# 如插入位置旁边的球可以消除的话，那么这种插入方法与直接将新球插入到与之颜色相同的球的旁边没有区别。
# 因此，这种操作不能得到比「插入新球与插入位置右侧的球颜色相同」更好的情况，得到更优解。读者可以结合以下例子理解。
# 例如：桌面上的球为 WWRRBBWW，手中的球为 WWRB，答案为 2。
# 操作方法如下：WWRRBBWW→WWRRBB(R)WW→WWRRB(B)BRWW→WWRRRWW→WWWW→""

# 细节
# 题目规定了如果在消除操作后，如果导致出现了新的连续三个或者三个以上颜色相同的球，则继续消除这些球，直到不再满足消除条件，
# 实际消除时我们可以利用栈的特性，每次遇到连续可以消除的球时，我们就将其从栈中弹出。
# 在实现中，我们可以在遍历桌面上的球时，使用列表维护遍历过的每种球的颜色和连续数量，从而通过一次遍历消除连续三个或者三个以上颜色相同的球。

# 在 Python中，因为对正则表达式的优化较好，也可以循环地使用正则表达式来消除连续三个或者三个以上颜色相同的球。
import re
from collections import deque
from itertools import product

def findMinStep(board: str, hand: str) -> int:
    def clean(s):
        # 消除桌面上需要消除的球
        n = 1
        while n:
            s, n = re.subn(r"(.)\1{2,}", "", s)
        return s

    hand = "".join(sorted(hand))

    # 初始化用队列维护的状态队列：其中的三个元素分别为桌面球状态、手中球状态和回合数
    queue = deque([(board, hand, 0)])

    # 初始化用哈希集合维护的已访问过的状态
    visited = {(board, hand)}

    while queue:
        cur_board, cur_hand, step = queue.popleft()
        print((len(cur_board) + 1, len(cur_hand)))
        # 笛卡尔积
        for i, j in product(range(len(cur_board) + 1), range(len(cur_hand))):
            # 第 1 个剪枝条件: 当前球的颜色和上一个球的颜色相同
            if j > 0 and cur_hand[j] == cur_hand[j - 1]:
                continue

            # 第 2 个剪枝条件: 只在连续相同颜色的球的开头位置插入新球
            if i > 0 and cur_board[i - 1] == cur_hand[j]:
                continue
            
            # 第 3 个剪枝条件: 只在以下两种情况放置新球
            choose = False
            
            #  - 第 1 种情况 : 当前球颜色与后面的球的颜色相同
            if i < len(cur_board) and cur_board[i] == cur_hand[j]:
                choose = True
                
            #  - 第 2 种情况 : 当前后颜色相同且与当前颜色不同时候放置球
            if 0 < i < len(cur_board) and cur_board[i - 1] == cur_board[i] and cur_board[i - 1] != cur_hand[j]:
                choose = True

            if choose:
                new_board = clean(cur_board[:i] + cur_hand[j] + cur_board[i:])
                new_hand = cur_hand[:j] + cur_hand[j + 1:]
                if not new_board:
                    return step + 1
                if (new_board, new_hand) not in visited:
                    queue.append((new_board, new_hand, step + 1))
                    visited.add((new_board, new_hand))

    return -1

print(findMinStep("WWRRBBWW", "WWRB"))

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


# 方法二：记忆化搜索
# 思路

# 记忆化搜索的核心思想跟方法一类似，核心思想还是搜索所有可能插入方案，找到最少的插入方案。
# 每次尝试选择一个手中的球将其插入到桌面上的任意两球之间，然后对桌面上的球进行检测并对连续相同颜色的球进行消除，
# 然后依次再进行插入和消除，直到桌面上的球全部消除或者手中的球全部插入后桌面上的球也无法消除为止。
# 假设当前桌面上有 n 个球，手中持有 m 个球，则此时一共有 Cm+nn×m!=An+mmC_{m+n}^{n} 种插入方法，
# 如果我们直接进行搜索所有的插入方法的话会超时，因此实际进行记忆化搜索时需要进行剪枝，剪枝的策略跟方法一类似，
# 当然实际中有很多可以进行剪枝的技巧。比如以下几个与方法一相同的减枝技巧：

# 第 1 个剪枝条件：手中颜色相同的球只需要考虑其中一个即可。
# 第 2 个剪枝条件：只在连续相同颜色的球的开头位置或者结尾位置插入新的颜色相同的球。
# 第 3 个剪枝条件：只考虑放置新球后有可能得到更优解的位置。
# 第 4 个剪枝条件：对于如果手中的球全部插入也无法满足新的消除，则我们直接进行中止。

import re
from functools import cache
from itertools import product

class Solution:
    def findMinStep(self, board: str, hand: str) -> int:
        ans = self.dfs(board, "".join(sorted(hand)))
        return ans if ans <= 5 else -1

    @cache
    def dfs(self, cur_board: str, cur_hand: str):
        if not cur_board:
            return 0
        
        res = 6
        for i, j in product(range(len(cur_board) + 1), range(len(cur_hand))):
            # 第 1 个剪枝条件: 手中颜色相同的球只需要考虑其中一个即可
            if j > 0 and cur_hand[j] == cur_hand[j - 1]:
                continue

            # 第 2 个剪枝条件: 只在连续相同颜色的球的开头位置插入新球
            if i > 0 and cur_board[i - 1] == cur_hand[j]:
                continue

            # 第 3 个剪枝条件: 只考虑放置新球后有可能得到更优解的位置
            choose = False
            #  - 第 1 种情况 : 当前球颜色与后面的球的颜色相同
            if i < len(cur_board) and cur_board[i] == cur_hand[j]:
                choose = True
            #  - 第 2 种情况 : 当前后颜色相同且与当前颜色不同时候放置球
            if 0 < i < len(cur_board) and cur_board[i - 1] == cur_board[i] and cur_board[i - 1] != cur_hand[j]:
                choose = True
            
            if choose:
                new_board = self.clean(cur_board[:i] + cur_hand[j] + cur_board[i:])
                new_hand = cur_hand[:j] + cur_hand[j + 1:]
                res = min(res, self.dfs(new_board, new_hand) + 1)
        return res

    @staticmethod
    def clean(s):
        n = 1
        while n:
            s, n = re.subn(r'(.)\1{2,}', '', s)
        return s