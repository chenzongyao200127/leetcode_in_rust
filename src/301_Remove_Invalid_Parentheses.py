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

# Since we do not know which brackets can be removed, we try all the options! We can use recursion.
# In the recursion, for each bracket, we can either use it or remove it.
# Recursion will generate all the valid parentheses strings but we want the ones with the least number of parentheses deleted.
# We can count the number of invalid brackets to be deleted and only generate the valid strings in the recusrion.
from typing import List
from collections import deque

# 超出内存限制
class Solution:
    def removeInvalidParentheses(self, s: str) -> List[str]:
        ans = set()
        s = [char for char in s]
        
        def is_valid(s):
            stack = []
            for char in s:
                if char == '(':
                    stack.append(char)
                elif char == ')':
                    if not stack:
                        return False
                    stack.pop()
            return not stack
        
        queue = deque([(s, 0)])
        min_removed = float('inf')
        
        while queue:
            curr_s, removed = queue.popleft()
            if removed > min_removed:
                break
            if is_valid(curr_s):
                ans.add("".join(curr_s))
                min_removed = removed
            else:
                for i in range(len(curr_s)):
                    if curr_s[i] in ['(', ')']:
                        new_s = curr_s[:i] + curr_s[i+1:]
                        queue.append((new_s, removed+1))
        
        return list(ans)
    
    
# 背景知识
# 有效的「括号」：题目输入的字符串由一系列「左括号」和「右括号」组成，但是有一些额外的括号，使得括号不能正确配对。
# 对于括号配对规则如果还不太清楚的读者，可以先完成问题「20. 有效的括号」。

# 可以一次遍历计算出多余的「左括号」和「右括号」： 根据括号匹配规则和根据求解「22. 括号生成」的经验，
# 我们知道：如果当前遍历到的「左括号」的数目严格小于「右括号」的数目则表达式无效。
# 因此，我们可以遍历一次输入字符串，统计「左括号」和「右括号」出现的次数。

# 当遍历到「左括号」的时候：
# 「左括号」数量加 1。
# 当遍历到「右括号」的时候：
# 如果此时「左括号」的数量不为 0，因为「右括号」可以与之前遍历到的「左括号」匹配，此时「左括号」出现的次数 −1；
# 如果此时「左括号」的数量为 0，「右括号」数量加 1。
# 通过这样的计数规则，得到的「左括号」和「右括号」的数量就是各自最少应该删除的数量。

# 方法一：回溯 + 剪枝
# 题目让我们删除括号使得剩下的括号匹配，要求我们删除最少的括号数，并且要求得到所有的结果。
# 我们可以使用回溯算法，尝试遍历所有可能的去掉非法括号的方案。

# 首先我们利用括号匹配的规则求出该字符串 s 中最少需要去掉的左括号的数目 lremove 和右括号的数目 rremove，
# 然后我们尝试在原字符串 s 中去掉 lremove 个左括号和 rremove 个右括号，然后检测剩余的字符串是否合法匹配，
# 如果合法匹配则我们则认为该字符串为可能的结果，我们利用回溯算法来尝试搜索所有可能的去除括号的方案。

# 在进行回溯时可以利用以下的剪枝技巧来增加搜索的效率：
# 我们从字符串中每去掉一个括号，则更新 lremove 或者 rremove，
# 当我们发现剩余未尝试的字符串的长度小于 lremove + rremove 时，则停止本次搜索。
# 当 lremove 和 rremove 同时为 000 时，则我们检测当前的字符串是否合法匹配，如果合法匹配则我们将其记录下来。
# 由于记录的字符串可能存在重复，因此需要对重复的结果进行去重，去重的办法有如下两种：

# 利用哈希表对最终生成的字符串去重。

# 我们在每次进行搜索时，如果遇到连续相同的括号我们只需要搜索一次即可，
# 比如当前遇到的字符串为 "(((())"，去掉前四个左括号中的任意一个，生成的字符串是一样的，均为 "((())"，
# 因此我们在尝试搜索时，只需去掉一个左括号进行下一轮搜索，不需要将前四个左括号都尝试一遍。
class Solution:
    def removeInvalidParentheses(self, s: str) -> List[str]:
        res = []
        lremove, rremove = 0, 0
        for c in s:
            if c == '(':
                lremove += 1
            elif c == ')':
                if lremove == 0:
                    rremove += 1
                else:
                    lremove -= 1

        def isValid(str):
            cnt = 0
            for c in str:
                if c == '(':
                    cnt += 1
                elif c == ')':
                    cnt -= 1
                    if cnt < 0:
                        return False
            return cnt == 0

        def helper(s, start, lremove, rremove):
            if lremove == 0 and rremove == 0:
                if isValid(s):
                    res.append(s)
                return

            for i in range(start, len(s)):
                if i > start and s[i] == s[i - 1]:
                    continue
                
                # 如果剩余的字符无法满足去掉的数量要求，直接返回
                if lremove + rremove > len(s) - i:
                    break
                
                # 尝试去掉一个左括号
                if lremove > 0 and s[i] == '(':
                    helper(s[:i] + s[i + 1:], i, lremove - 1, rremove);
                    
                # 尝试去掉一个右括号
                if rremove > 0 and s[i] == ')':
                    helper(s[:i] + s[i + 1:], i, lremove, rremove - 1);

        helper(s, 0, lremove, rremove)
        return res


# 方法二：广度优先搜索 ⭐
# 思路与算法

# 注意到题目中要求最少删除，这样的描述正是广度优先搜索算法应用的场景，并且题目也要求我们输出所有的结果。
# 我们在进行广度优先搜索时每一轮删除字符串中的 1 个括号，直到出现合法匹配的字符串为止，此时进行轮转的次数即为最少需要删除括号的个数。

# 我们进行广度优先搜索时，每次保存上一轮搜索的结果，
# 然后对上一轮已经保存的结果中的每一个字符串尝试所有可能的删除一个括号的方法，
# 然后将保存的结果进行下一轮搜索。在保存结果时，我们可以利用哈希表对上一轮生成的结果去重，从而提高效率。

class Solution:
    def removeInvalidParentheses(self, s: str) -> List[str]:
        def isValid(s):
            count = 0
            for c in s:
                if c == '(':
                    count += 1
                elif c == ')':
                    count -= 1
                    if count < 0:
                        return False
            return count == 0

        ans = []
        currSet = set([s])

        while True:
            for ss in currSet:
                if isValid(ss):
                    ans.append(ss)

            if len(ans) > 0:
                return ans
            
            nextSet = set()
            for ss in currSet:
                for i in range(len(ss)):
                    if i > 0 and ss[i] == s[i - 1]:
                        continue
                    if ss[i] == '(' or ss[i] == ')':
                        nextSet.add(ss[:i] + ss[i + 1:])
                        
            currSet = nextSet
            
        return ans

# 方法三：枚举状态子集
# 思路与算法
# 首先我们利用括号匹配的规则求出该字符串 s 中最少需要去掉的左括号的数目 lremove 和右括号的数目 rremove，
# 然后我们利用状态子集求出字符串 s 中所有的左括号去掉 lremove的左括号的子集，和所有的右括号去掉 rremove
# 个右括号的子集，依次枚举这两种子集的组合，检测组合后的字符串是否合法匹配，如果字符串合法则记录，最后我们利用哈希表对结果进行去重。
class Solution:
    def removeInvalidParentheses(self, s: str) -> List[str]:
        def checkValid(str, lmask, left, rmask, right):
            pos1, pos2 = 0, 0
            cnt = 0

            for i in range(len(str)):
                if pos1 < len(left) and i == left[pos1]:
                    if lmask & (1 << pos1) == 0:
                        cnt += 1
                    pos1 += 1
                elif pos2 < len(right) and i == right[pos2]:
                    if rmask & (1 << pos2) == 0:
                        cnt -= 1
                        if cnt < 0:
                            return False
                    pos2 += 1

            return cnt == 0

        def recoverStr(lmask, left, rmask, right):
            pos1, pos2 = 0, 0
            res = ""

            for i in range(len(s)):
                if pos1 < len(left) and i == left[pos1]:
                    if lmask & (1 << pos1) == 0:
                        res += s[i]
                    pos1 += 1
                elif pos2 < len(right) and i == right[pos2]:
                    if rmask & (1 << pos2) == 0:
                        res += s[i]
                    pos2 += 1
                else:
                    res += s[i]

            return res

        def countBit(x):
            res = 0
            while x != 0:
                x = x & (x - 1)
                res += 1
            return res

        lremove, rremove = 0, 0
        left, right = [], []
        ans = []
        cnt = set()

        for i in range(len(s)):
            if s[i] == '(':
                left.append(i)
                lremove += 1
            elif s[i] == ')':
                right.append(i)
                if lremove == 0:
                    rremove += 1
                else:
                    lremove -= 1

        m, n = len(left), len(right)
        maskArr1, maskArr2 = [], []
        for i in range(1 << m):
            if countBit(i) != lremove:
                continue
            maskArr1.append(i)
        for i in range(1 << n):
            if countBit(i) != rremove:
                continue
            maskArr2.append(i)
        for mask1 in maskArr1:
            for mask2 in maskArr2:
                if checkValid(s, mask1, left, mask2, right):
                    cnt.add(recoverStr(mask1, left, mask2, right))
            
        return [val for val in cnt]
