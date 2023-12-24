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



class Solution:
    def incremovableSubarrayCount(self, nums: List[int]) -> int:
        n = len(nums)
        if n == 0:
            return 0

        # Arrays to store the maximum to the left and the minimum to the right
        max_left = [None] * n
        min_right = [None] * n

        # Fill in the max_left array
        max_so_far = float('-inf')
        for i in range(n):
            max_so_far = max(max_so_far, nums[i])
            max_left[i] = max_so_far

        # Fill in the min_right array
        min_so_far = float('inf')
        for i in range(n - 1, -1, -1):
            min_so_far = min(min_so_far, nums[i])
            min_right[i] = min_so_far

        ans = 0
        for i in range(n):
            if (i == 0 or max_left[i - 1] < nums[i]) and (i == n - 1 or nums[i] < min_right[i + 1]):
                ans += 1

        return ans
    


class Solution:
    def numberGame(self, nums: List[int]) -> List[int]:
        nums.sort()
        arr = []
        for _ in range(0, len(nums)//2):
            arr.append(nums[1])
            arr.append(nums[0])
            nums = nums[2:]
        
        return arr
    
    

class Solution:
    def maximizeSquareArea(self, m: int, n: int, hFences: List[int], vFences: List[int]) -> int:
        hFences.sort()
        vFences.sort()
        a = [1] + hFences + [m]
        b = [1] + vFences + [n]
        MOD = 10 ** 9 + 7
        
        # print((a, b))
        
        rows = set()
        for i in range(len(a)-1):
            for j in range(i + 1, len(a)):
                rows.add(a[j] - a[i])
        
        cols = set()
        for i in range(len(b)-1):
            for j in range(i + 1, len(b)):
                cols.add(b[j] - b[i])

        # print(rows)
        # print(cols)
        union_set = rows & cols
        if not union_set:
            return -1
        else:
            x = max(union_set)
            return (x * x) % MOD        
        
class Solution:
    def minimumCost(self, source: str, target: str, original: List[str], changed: List[str], cost: List[int]) -> int:
        import string

        # Initialize the graph with high costs
        graph = {c1: {c2: float('inf') for c2 in string.ascii_lowercase} for c1 in string.ascii_lowercase}

        # Set the cost of self-transformation to 0
        for c in string.ascii_lowercase:
            graph[c][c] = 0

        # Add the transformations to the graph
        for o, c, co in zip(original, changed, cost):
            if co < graph[o][c]:
                graph[o][c] = co

        # Floyd-Warshall Algorithm to find all pairs shortest paths
        for k in string.ascii_lowercase:
            for i in string.ascii_lowercase:
                for j in string.ascii_lowercase:
                    graph[i][j] = min(graph[i][j], graph[i][k] + graph[k][j])

        # Calculate the total cost of transformation
        total_cost = 0
        for s, t in zip(source, target):
            if graph[s][t] == float('inf'):
                return -1  # Transformation not possible
            total_cost += graph[s][t]

        return total_cost
    
s = Solution()
ans = s.minimumCost(source = "abcd", target = "acbe", original = ["a","b","c","c","e","d"], changed = ["b","c","b","e","b","e"], cost = [2,5,5,1,2,20])
print(ans)

s = Solution()
ans = s.minimumCost(source = "aaaa", target = "bbbb", original = ["a","c"], changed = ["c","b"], cost = [1,2])
print(ans)

s = Solution()
ans = s.minimumCost(source = "abcd", target = "abce", original = ["a"], changed = ["e"], cost = [10000])
print(ans)