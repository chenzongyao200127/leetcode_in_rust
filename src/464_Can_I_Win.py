# 464_Can_I_Win
# https://leetcode.cn/problems/can-i-win/

# 在 "100 game" 这个游戏中，两名玩家轮流选择从 1 到 10 的任意整数，累计整数和，
# 先使得累计整数和 达到或超过  100 的玩家，即为胜者。

# 如果我们将游戏规则改为 “玩家 不能 重复使用整数” 呢？

# 例如，两个玩家可以轮流从公共整数池中抽取从 1 到 15 的整数（不放回）
# 直到累计整数和 >= 100。

# 给定两个整数 maxChoosableInteger （整数池中可选择的最大数）和 desiredTotal（累计和），
# 若先出手的玩家能稳赢则返回 true ，否则返回 false 。假设两位玩家游戏时都表现 最佳 。

 

# 示例 1：

# 输入：maxChoosableInteger = 10, desiredTotal = 11
# 输出：false
# 解释：
# 无论第一个玩家选择哪个整数，他都会失败。
# 第一个玩家可以选择从 1 到 10 的整数。
# 如果第一个玩家选择 1，那么第二个玩家只能选择从 2 到 10 的整数。
# 第二个玩家可以通过选择整数 10（那么累积和为 11 >= desiredTotal），从而取得胜利.
# 同样地，第一个玩家选择任意其他整数，第二个玩家都会赢。
# 示例 2:

# 输入：maxChoosableInteger = 10, desiredTotal = 0
# 输出：true
# 示例 3:

# 输入：maxChoosableInteger = 10, desiredTotal = 1
# 输出：true
 

# 提示:

# 1 <= maxChoosableInteger <= 20
# 0 <= desiredTotal <= 300


# 这是一个博弈论的问题。对于给定的maxChoosableInteger和desiredTotal，
# 我们需要确定对于先手玩家是否有稳赢的策略。这可以通过递归或记忆化搜索来解决。

# 递归函数的核心思路是考虑当前可选整数集合state和当前的累计和total，
# 返回当前玩家是否可以赢。

# 为了优化递归的效率，我们使用记忆化搜索，
# 即用一个字典memo来记录已经计算过的(state, total)对应的结果。

# 具体代码如下：


def canIWin(maxChoosableInteger, desiredTotal):
    # 如果可选择的整数总和小于desiredTotal，则先手玩家必输
    if (1 + maxChoosableInteger) * maxChoosableInteger / 2 < desiredTotal:
        return False
    
    if desiredTotal <= 0:
        return True

    def helper(state, total, memo):
        if total <= 0:
            return False
        
        if state in memo:
            return memo[state]
        
        for i in range(1, maxChoosableInteger + 1):
            if not state & (1 << i) and not helper(state | (1 << i), total - i, memo):
                    memo[state] = True
                    return True
        memo[state] = False
        return False

    return helper(0, desiredTotal, {})

# 示例
print(canIWin(10, 11))  # 输出：false
print(canIWin(10, 0))  # 输出：true
print(canIWin(10, 1))  # 输出：true


# 注意：state用位操作表示整数集合
# 例如当maxChoosableInteger为10时，
# 如果state的第3位和第5位是1，则表示整数3和5已经被选择过了。


def canIWin(maxChoosableInteger, desiredTotal):
    if maxChoosableInteger >= desiredTotal: return True
    if (1 + maxChoosableInteger) * maxChoosableInteger / 2 < desiredTotal: return False

    memo = {}

    # 递归函数
    def dp(state, total):
        if state in memo: 
            return memo[state]
        
        for i in range(maxChoosableInteger, 0, -1):
            # 检查当前数字是否已被选择
            if not (state & (1 << (i - 1))):
                # 如果选择这个数字使得总和达到或超过desiredTotal，或者选择这个数字后，对手输了
                # 那么当前玩家将赢得比赛
                if total + i >= desiredTotal or not dp(state | (1 << (i - 1)), total + i):
                    memo[state] = True
                    return True
        
        memo[state] = False
        return False

    return dp(0, 0)

# 示例
print(canIWin(10, 11))  # 输出：false
print(canIWin(10, 0))  # 输出：true
print(canIWin(10, 1))  # 输出：


# 官解
class Solution:
    def canIWin(self, maxChoosableInteger: int, desiredTotal: int) -> bool:
        @cache
        def dfs(usedNumbers: int, currentTotal: int) -> bool:
            for i in range(maxChoosableInteger):
                if (usedNumbers >> i) & 1 == 0:
                    if currentTotal + i + 1 >= desiredTotal or \
                        not dfs(usedNumbers | (1 << i), currentTotal + i + 1):
                        return True
            return False

        return (1 + maxChoosableInteger) * maxChoosableInteger // 2 >= desiredTotal and dfs(0, 0)