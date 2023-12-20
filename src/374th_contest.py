# 373th_contest

from typing import List

class Solution:
    def findPeaks(self, mountain: List[int]) -> List[int]:
        peaks = []
        for i in range(1, len(mountain) - 1):
            if mountain[i] > mountain[i - 1] and mountain[i] > mountain[i + 1]:
                peaks.append(i)
        return peaks

# # Example usage
# # mountain = [1, 3, 2, 4, 1, 5, 3, 2, 4, 6]
# mountain = [1,4,3,8,5]
# s = Solution()
# ans = s.findPeaks(mountain)
# print(ans)

# 给你一个下标从 0 开始的整数数组 coins，表示可用的硬币的面值，以及一个整数 target 。
# 如果存在某个 coins 的子序列总和为 x，那么整数 x 就是一个 可取得的金额 。
# 返回需要添加到数组中的 任意面值 硬币的 最小数量 ，使范围 [1, target] 内的每个整数都属于 可取得的金额 。
# 数组的 子序列 是通过删除原始数组的一些（可能不删除）元素而形成的新的 非空 数组，删除过程不会改变剩余元素的相对位置。

class Solution:
    def minimumAddedCoins(self, coins: List[int], target: int) -> int:
        coins.sort()
        ans, total = 0, 0
        for coin in coins:
            while coin > total + 1 and total < target:
                total += total + 1
                ans += 1
            total += coin
            if total >= target:
                break
        while total < target:
            total += total + 1
            ans += 1
        return ans


class Solution:
    def minimumAddedCoins(self, coins: List[int], target: int) -> int:
        coins.sort()
        ans, s, i = 0, 1, 0
        while s <= target:
            if i < len(coins) and coins[i] <= s:
                s += coins[i]
                i += 1
            else:
                s *= 2  # 必须添加 s
                ans += 1
        return ans

# 作者：灵茶山艾府
# 链接：https://leetcode.cn/problems/minimum-number-of-coins-to-be-added/
# 来源：力扣（LeetCode）
# 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。

# # Example
# coins = [1,1,1]
# target = 20
# s = Solution()
# ans = s.minimumAddedCoins(coins, target)
# print(ans)


# class Solution:
#     def countCompleteSubstrings(self, word: str, k: int) -> int:
#         def is_valid_window(char_count, k):
#             return all(count == k for count in char_count.values())
        
#         def is_alphabetically_close(s):
#             for i in range(len(s) - 1):
#                 if abs(ord(s[i]) - ord(s[i + 1])) > 2:
#                     return False
#             return True

#         n = len(word)
#         complete_substring_count = 0

#         for window_size in range(k, n + 1, k):
#             char_count = {}
#             for i in range(window_size):
#                 char_count[word[i]] = char_count.get(word[i], 0) + 1

#             if is_valid_window(char_count, k) and is_alphabetically_close(word[:window_size]):
#                 complete_substring_count += 1

#             for right in range(window_size, n):
#                 left = right - window_size
#                 char_count[word[right]] = char_count.get(word[right], 0) + 1
#                 char_count[word[left]] -= 1

#                 if char_count[word[left]] == 0:
#                     del char_count[word[left]]

#                 if is_valid_window(char_count, k) and is_alphabetically_close(word[left+1:right+1]):
#                     complete_substring_count += 1

#         return complete_substring_count

from collections import Counter

class Solution:
    def countCompleteSubstrings(self, word: str, k: int) -> int:
        n = len(word)
        perfect_count = 0

        # Function to check if the substring is valid
        def is_valid(substring):
            return all(substring.count(char) == k for char in set(substring)) and \
                all(abs(ord(substring[j]) - ord(substring[j + 1])) <= 2 for j in range(len(substring) - 1))

        # Iterate over the string to find valid substrings
        for i in range(n):
            for j in range(i + k, n + 1):
                substring = word[i:j]
                if is_valid(substring):
                    perfect_count += 1

        return perfect_count

# Example
# word = "aaabbbccc"
# k = 3
# word = "igigee"
# k = 2
# s = Solution()
# ans = s.countCompleteSubstrings(word, k)
# print(ans)

# word = "aaabbbccc"
# k = 3
# ans = s.countCompleteSubstrings(word, k)
# print(ans)

# word = "ba"
# k = 1
# ans = s.countCompleteSubstrings(word, k)
# print(ans)


class Solution:
    def numberOfSequence(self, n: int, sick: List[int]) -> int:
        MOD = 10**9 + 7
        
        if sick == [0] or sick == [n-1]:
            return 1

        # Function to calculate the number of sequences for each segment
        def calculate_sequences(segment_length):
            if segment_length <= 1:
                return 1
            return pow(2, segment_length - 1, MOD)

        # Add boundary elements to the sick list
        sick = [-1] + sick + [n]
        # print(sick)

        total_sequences = 1
        for i in range(1, len(sick)):
            # Calculate the length of the segment
            # print(sick[i - 1], sick[i])
            if sick[i-1] == -1 or sick[i] == n:
                if (sick[i] - sick[i-1] - 1) == 0:
                    total_sequences += 0
                else:
                    total_sequences += 1
            else:
                segment_length = sick[i] - sick[i-1] - 1
            # Multiply the number of sequences for each segment
                total_sequences *= calculate_sequences(segment_length)
            total_sequences %= MOD

        return total_sequences

    
s = Solution()
ans = s.numberOfSequence(n = 5, sick = [0,4])
print(ans) # 4

s = Solution()
ans = s.numberOfSequence(n = 4, sick = [1])
print(ans) # 3

s = Solution()
ans = s.numberOfSequence(n = 5, sick = [0])
print(ans) # 1