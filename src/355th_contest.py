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


# 给你一个下标从 0 开始、长度为 n 的数组 usageLimits 。

# 你的任务是使用从 0 到 n - 1 的数字创建若干组，
# 并确保每个数字 i 在 所有组 中使用的次数总共不超过 usageLimits[i] 次。此外，还必须满足以下条件：

# 每个组必须由 不同 的数字组成，也就是说，单个组内不能存在重复的数字。
# 每个组（除了第一个）的长度必须 严格大于 前一个组。

# 在满足所有条件的情况下，以整数形式返回可以创建的最大组数。
