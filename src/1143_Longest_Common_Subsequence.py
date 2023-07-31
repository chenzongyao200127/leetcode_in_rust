# 1143_Longest_Common_Subsequence
# https://leetcode.cn/problems/longest-common-subsequence/description/

class Solution:
    def longestCommonSubsequence(self, text1: str, text2: str) -> int:
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
        
        