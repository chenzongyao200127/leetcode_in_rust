def split_words(words, separator):
    result = []
    for word in words:
        splits = word.split(separator)
        for split in splits:
            if split != "":
                result.append(split)
    return result


def maxElemAfterOp(nums):
    n = len(nums)
    for i in range(n - 1, 0, -1):
        if nums[i - 1] <= nums[i]:
            nums[i - 1] += nums[i]
            nums.pop(i)
    return max(nums)


from collections import Counter
from typing import List
import collections

class Solution:
    def countPalindromePaths(self, parent: List[int], s: str) -> int:
        n = len(parent)
        edges = collections.defaultdict(list)
        for i in range(1, n):
            edges[parent[i]].append(i)

        res = 0
        for i in range(n):
            counter = collections.Counter()
            stack = [(i, counter.copy())]
            while stack:
                node, cnt = stack.pop()
                cnt[s[node]] += 1
                cnt[s[node]] %= 2
                if sum(val for val in cnt.values()) <= 1:
                    res += 1
                for child in edges[node]:
                    stack.append((child, cnt.copy()))
        return res

solution = Solution()
print(solution.countPalindromePaths([-1,0,0,1,1,2], "acaabc"))  
print(solution.countPalindromePaths([-1,0,0,0,0], "aaaaa"))  