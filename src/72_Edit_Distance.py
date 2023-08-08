# 72_Edit_Distance
# https://leetcode.cn/problems/edit-distance/description/

# 给你两个单词 word1 和 word2， 请返回将 word1 转换成 word2 所使用的最少操作数  。

# 你可以对一个单词进行如下三种操作：

# 插入一个字符
# 删除一个字符
# 替换一个字符
 

class Solution:
    def minDistance(self, word1: str, word2: str) -> int:
        dp = [[0] * (len(word2) + 1) for _ in range(len(word1) + 1)]
        
        for i in range(len(word1)):
            dp[i+1][0] = i+1
            
        for i in range(len(word2)):
            dp[0][i+1] = i+1
            
        for i in range(1, len(word1)+1):
            for j in range(1, len(word2)+1):
                x = dp[i][j-1] + 1
                y = dp[i-1][j] + 1
                if word1[i-1] == word2[j-1]:
                    dp[i][j] = dp[i-1][j-1]
                else:
                    dp[i][j] = min(x, y, dp[i-1][j-1]+1)

        return dp[-1][-1]
        

# 评论区大佬
class Solution:
    def minDistance(self, s: str, t: str) -> int:
        n, m = len(s), len(t)
        @cache
        def dfs(i, j):
            if i < 0: return j + 1
            if j < 0: return i + 1
            if s[i] == t[j]: return dfs(i - 1, j - 1)
            return min(dfs(i - 1, j), dfs(i, j - 1), dfs(i - 1, j - 1)) + 1
        return dfs(n - 1, m - 1)

# 作者：灵茶山艾府
# 链接：https://leetcode.cn/problems/edit-distance/solutions/2133222/jiao-ni-yi-bu-bu-si-kao-dong-tai-gui-hua-uo5q/
# 来源：力扣（LeetCode）
# 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。