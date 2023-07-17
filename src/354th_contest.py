# Problem No.2
class Solution:
    def maximumBeauty(self, nums: List[int], k: int) -> int:
        nums.sort()
        ans = left = 0
        for right, x in enumerate(nums):
            while x - nums[left] > k * 2:
                left += 1
            ans = max(ans, right - left + 1)
        return ans



# Problem No.3
from collections import Counter
from typing import List

class Solution:
    def minimumIndex(self, nums: List[int]) -> int:
        mode, total = Counter(nums).most_common(1)[0]
        freq1 = 0
        for i, x in enumerate(nums):
            freq1 += x == mode
            if freq1 * 2 > i + 1 and (total - freq1) * 2 > len(nums) - i - 1:
                return i
        return -1


# Problem No.4
# 给你一个字符串 word 和一个字符串数组 forbidden 。
# 如果一个字符串不包含 forbidden 中的任何字符串，我们称这个字符串是 合法 的。
# 请你返回字符串 word 的一个 最长合法子字符串 的长度。
# 子字符串 指的是一个字符串中一段连续的字符，它可以为空。

# 1 <= word.length <= 105
# word 只包含小写英文字母。
# 1 <= forbidden.length <= 105
# 1 <= forbidden[i].length <= 10
# forbidden[i] 只包含小写英文字母。

class Solution:
    def longestValidSubstring(self, word: str, forbidden: List[str]) -> int:
        fb = set(forbidden)
        ans = left = 0
        for right in range(len(word)):
            for i in range(right, max(right - 10, left - 1), -1):
                if word[i: right + 1] in fb:
                    left = i + 1  # 当子串右端点 >= right 时，合法子串一定不能包含 word[i]
                    break
            ans = max(ans, right - left + 1)
        return ans
