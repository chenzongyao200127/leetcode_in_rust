# 132_Palindrome_Partitioning_II
# https://leetcode.cn/problems/palindrome-partitioning-ii/

# 给你一个字符串 s，请你将 s 分割成一些子串，使每个子串都是回文。
# 返回符合要求的 最少分割次数 。

# 示例 1：
# 输入：s = "aab"
# 输出：1
# 解释：只需一次分割就可将 s 分割成 ["aa","b"] 这样两个回文子串。

# 示例 2：
# 输入：s = "a"
# 输出：0

# 示例 3：
# 输入：s = "ab"
# 输出：1

from typing import List

class Solution:
    def partition(self, s: str) -> List[List[str]]:
        ans = []
        ret = []
        n = s.len()
        
        dp = [[True] * n for _ in range(n)]
        for i in range(n-1, -1, -1):
            for j in range(i+1, n):
                dp[i][j] = (dp[i+1][j-1] and s[i] == s[j])
        
        def dfs(i: int):
            if i == n:
                ret.append(ans[:])
                return
            
            for j in range(i, n):
                if dp[i][j]:
                    ans.append(s[i:j+1])
                    dfs(j + 1)
                    ans.pop()
                    
        dfs(0)
        return ret
                
                    