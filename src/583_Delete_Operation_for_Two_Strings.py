# 583_Delete_Operation_for_Two_Strings
# https://leetcode.cn/problems/delete-operation-for-two-strings/description/

# 给定两个单词 word1 和 word2 ，返回使得 word1 和  word2 相同所需的最小步数。
# 每步 可以删除任意一个字符串中的一个字符。

class Solution:
    def minDistance(self, word1: str, word2: str) -> int:
        def longestCommonSubsequence(text1: str, text2: str) -> int:
            s1 = list(text1)
            s2 = list(text2)
            
            dp = [[0] * (len(s1)+1) for _ in range(len(s2)+1)]
            
            for i in range(len(s1)+1):
                dp[0][i] = 0
            for i in range(len(s2)+1):
                dp[i][0] = 0
                
            for i in range(1, len(s2)+1):
                for j in range(1, len(s1)+1):
                    if s2[i-1] == s1[j-1]:
                        dp[i][j] = dp[i-1][j-1] + 1
                    else:
                        dp[i][j] = max(dp[i-1][j], dp[i][j-1])
                        
            return dp[len(s2)][len(s1)]
        
        lcs = longestCommonSubsequence(word1, word2)
        return (len(word1) + len(word2) - 2 * lcs)