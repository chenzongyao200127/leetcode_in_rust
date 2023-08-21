# 139_Word_Break
# https://leetcode.cn/problems/word-break/

# Example 1:
# Input: s = "leetcode", wordDict = ["leet","code"]
# Output: true
# Explanation: Return true because "leetcode" can be segmented as "leet code".

# Example 2:
# Input: s = "applepenapple", wordDict = ["apple","pen"]
# Output: true
# Explanation: Return true because "applepenapple" can be segmented as "apple pen apple".
# Note that you are allowed to reuse a dictionary word.

# Example 3:
# Input: s = "catsandog", wordDict = ["cats","dog","sand","and","cat"]
# Output: false

from typing import List
class Solution:
    def wordBreak(self, s: str, wordDict: List[str]) -> bool:
        set = set(wordDict)
        dp = [False] * (len(s) + 1)
        dp[0] = True
        for i in range(0, len(s)+1):
            for j in range(i):
                if dp[j] == True:
                    print(s[j:i])
                    if s[j:i] in set:
                        dp[i] = True
        return dp[-1]
                        
s = Solution()
s.wordBreak("a", ["a"])



# 优化
from typing import List

class Solution:
    def wordBreak(self, s: str, wordDict: List[str]) -> bool:
        wordSet = set(wordDict)
        dp = [False] * (len(s) + 1)
        dp[0] = True

        for i in range(1, len(s)+1):
            for j in range(i):
                if dp[j]:
                    if s[j:i] in wordSet:
                        dp[i] = True
                        break  # No need to check other substrings starting from j if found

        return dp[-1]
