# 140_Word_Break_II
# https://leetcode.cn/problems/word-break-ii/

# 示例 1：

# 输入:s = "catsanddog", wordDict = ["cat","cats","and","sand","dog"]
# 输出:["cats and dog","cat sand dog"]

# 示例 2：
# 输入:s = "pineapplepenapple", wordDict = ["apple","pen","applepen","pine","pineapple"]
# 输出:["pine apple pen apple","pineapple pen apple","pine applepen apple"]
# 解释: 注意你可以重复使用字典中的单词。

# 示例 3：
# 输入:s = "catsandog", wordDict = ["cats","dog","sand","and","cat"]
# 输出:[]

from typing import List
class Solution:
    def wordBreak(self, s: str, wordDict: List[str]) -> List[str]:
        wordSet = set(wordDict)
        def helper(s: str) -> bool:
            dp = [False] * (len(s) + 1)
            dp[0] = True

            for i in range(1, len(s)+1):
                for j in range(i):
                    if dp[j]:
                        if s[j:i] in wordSet:
                            dp[i] = True
                            break
                            
            return dp
        
        dp = helper(s)
        
        if not dp[-1]:
            return []
        
        res = []
        def dfs(idx, l):
            if idx == len(s):
                res.append(" ".join(l))
                return
            
            for i in range(idx, len(s)):
                if s[idx:i+1] in wordDict and dp[idx]:
                    l.append(s[idx:i+1])
                    dfs(i+1, l)
                    l.pop()
        
        dfs(0, [])
        return res
    


from typing import List

class Solution:
    def wordBreak(self, s: str, wordDict: List[str]) -> List[str]:
        wordSet = set(wordDict)
        
        def canBreak(s: str) -> List[bool]:
            dp = [False] * (len(s) + 1)
            dp[0] = True

            for i in range(1, len(s)+1):
                for j in range(i):
                    if dp[j] and s[j:i] in wordSet:
                        dp[i] = True
                        break
                        
            return dp

        dp = canBreak(s)
        
        if not dp[-1]:
            return []

        res = []
        def dfs(idx, l):
            if idx == len(s):
                res.append(" ".join(l))
                return

            for i in range(idx, len(s)):
                if s[idx:i+1] in wordSet and dp[i+1]:
                    l.append(s[idx:i+1])
                    dfs(i+1, l)
                    l.pop()

        dfs(0, [])
        return res
    
    
# Certainly, the given solution can be further optimized using memoization 
# to reduce redundant computations during the depth-first search. 

# The primary optimization will come from memoizing the results of the `dfs` function for each index. 
# Once you've computed the results for a specific index, you can reuse them instead of recalculating them.

# Here's how you can optimize the code using memoization:

from typing import List

class Solution:
    def wordBreak(self, s: str, wordDict: List[str]) -> List[str]:
        # Convert wordDict to a set for O(1) time complexity lookups.
        wordSet = set(wordDict)

        # This function checks if the string s can be segmented.
        def canBreak(s: str) -> List[bool]:
            # dp[i] will be True if s[0:i] can be segmented using words from wordSet.
            dp = [False] * (len(s) + 1)
            dp[0] = True

            for i in range(1, len(s)+1):
                for j in range(i):
                    # If the string up to j can be segmented and the substring s[j:i] is in wordSet,
                    # then the string up to i can be segmented.
                    if dp[j] and s[j:i] in wordSet:
                        dp[i] = True
                        break

            return dp

        # Check if the entire string can be segmented.
        dp = canBreak(s)

        # If the last element of dp is False, the string cannot be segmented.
        if not dp[-1]:
            return []

        # Using memoization to avoid redundant calculations.
        memo = {}

        # This function finds all possible segmentations of the string starting from idx.
        def dfs(idx):
            # Base case: if idx is the end of the string, return an empty string.
            if idx == len(s):
                return [""]
            
            # If this index has been computed before, return its value.
            if idx in memo:
                return memo[idx]

            # To store all possible sentences starting from idx.
            sentences = []

            for i in range(idx, len(s)):
                word = s[idx:i+1]
                # If the current word is in wordSet and the rest of the string can be segmented,
                # then proceed with the dfs.
                if word in wordSet and dp[i+1]:
                    rest_sentences = dfs(i+1)
                    for sentence in rest_sentences:
                        # Add the current word to each of the sentences formed from the rest of the string.
                        if sentence:
                            sentences.append(word + " " + sentence)
                        else:
                            sentences.append(word)

            # Save the results in memo and return them.
            memo[idx] = sentences
            return sentences

        # Start the dfs from the beginning of the string.
        return dfs(0)


# In this optimized version:
# 1. Introduced a `memo` dictionary to store the results of the `dfs` function for each index.
# 2. Modified the `dfs` function to directly return sentences for each index instead of appending them to a global result list.
# 3. Utilized memoization inside the `dfs` function to avoid redundant calculations.

# By adding memoization, the running time of the `dfs` function is significantly reduced, 
# as it doesn't repeatedly compute results for the same indices. 
# This optimization will be especially noticeable for larger inputs.