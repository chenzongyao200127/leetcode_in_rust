import re
from collections import Counter
from typing import List
import math


class Solution:
    def areaOfMaxDiagonal(self, dimensions: List[List[int]]) -> int:
        max_diagonal = 0
        max_area = 0

        for dimension in dimensions:
            length, width = dimension
            diagonal = math.sqrt(length ** 2 + width ** 2)
            area = length * width
            if diagonal > max_diagonal or (diagonal == max_diagonal and area > max_area):
                max_diagonal = diagonal
                max_area = area

        return max_area


class Solution:
    def minMovesToCaptureTheQueen(self, a: int, b: int, c: int, d: int, e: int, f: int) -> int:
        rook_can_capture = (a == e or b == f) and not ((a == e == c and (
            (b < d < f) or (f < d < b))) or (b == f == d and ((a < c < e) or (e < c < a))))

        bishop_can_capture = False
        if abs(c - e) == abs(d - f):
            step_x = 1 if e > c else -1
            step_y = 1 if f > d else -1
            x, y = c + step_x, d + step_y
            while 0 <= x < 8 and 0 <= y < 8 and (x, y) != (e, f):
                if (x, y) == (a, b):
                    break
                x, y = x + step_x, y + step_y
            else:
                bishop_can_capture = True

        if rook_can_capture or bishop_can_capture:
            return 1
        else:
            return 2


class Solution:
    def maximumSetSize(self, nums1: List[int], nums2: List[int]) -> int:
        n = len(nums1)
        half_n = n // 2

        # 计算两个数组中每个元素的出现次数
        freq1 = Counter(nums1)
        freq2 = Counter(nums2)
        d1 = 0
        d2 = 0
        # 移除各自数组中的重复元素（保留一个）
        for num in set(nums1):
            if freq1[num] > 1:
                d1 += freq1[num] - 1
                freq1[num] = 1
        for num in set(nums2):
            if freq2[num] > 1:
                d2 += freq2[num] - 1
                freq2[num] = 1

        # 计算目前各数组中的元素总数
        current_count1 = sum(freq1.values())
        current_count2 = sum(freq2.values())

        # 如果移除重复元素后已达到目标，直接计算结果
        if current_count1 <= half_n and current_count2 <= half_n:
            result = len(set([num for num in freq1 if freq1[num] > 0]).union(
                set([num for num in freq2 if freq2[num] > 0])))
            return result

        # 计算两个数组的交集
        intersection = set(nums1).intersection(set(nums2))

        # 移除交集中的元素
        for num in intersection:
            if current_count1 > current_count2:
                freq1[num] = 0
                current_count1 -= 1
            else:
                freq2[num] = 0
                current_count2 -= 1

        # 计算最终集合中的唯一元素数量
        result = len(set([num for num in freq1 if freq1[num] > 0]).union(
            set([num for num in freq2 if freq2[num] > 0])))

        if current_count1 > half_n:
            result -= (current_count1 - half_n)

        if current_count2 > half_n:
            result -= (current_count2 - half_n)

        return result


class Solution:
    def maxFrequencyElements(self, nums: List[int]) -> int:
        # Count the frequency of each number in the array
        frequency = Counter(nums)

        # Find the maximum frequency
        max_freq = max(frequency.values())

        # Calculate the sum of frequencies of elements with maximum frequency
        return sum(freq for freq in frequency.values() if freq == max_freq)


# class Solution:
#     def beautifulIndices(self, s: str, a: str, b: str, k: int) -> List[int]:
#         def find_occurrences(sub, string):
#             start = 0
#             while start < len(string):
#                 start = string.find(sub, start)
#                 if start == -1:
#                     break
#                 yield start
#                 start += 1

#         # Find all occurrences of 'a' and 'b' in 's'
#         indices_a = list(find_occurrences(a, s))
#         indices_b = list(find_occurrences(b, s))

#         # Check for beautiful indices
#         beautiful_indices = []
#         for i in indices_a:
#             for j in indices_b:
#                 if abs(j - i) <= k:
#                     beautiful_indices.append(i)
#                     break

#         return sorted(set(beautiful_indices))


class Solution:
    def findMaximumNumber(self, k: int, x: int) -> int:
        def value_of_num(num):
            value = 0
            pos = 1
            while num > 0:
                if pos % x == 0 and (num & 1):
                    value += 1
                num >>= 1
                pos += 1
            return value

        total_value = 0
        num = 0

        while total_value <= k:
            num += 1
            total_value += value_of_num(num)
            if total_value > k:
                return num - 1

        return num


class Solution:
    def findMaximumNumber(self, k: int, x: int) -> int:
        # Function to count the value of a number
        def count_value(num):
            value = 0
            bit_index = 1
            while num > 0:
                if num % 2 == 1 and bit_index % x == 0:
                    value += 1
                num //= 2
                bit_index += 1
            return value

        # Function to calculate total value up to a number
        def total_value_up_to(num):
            total = 0
            for i in range(1, num + 1):
                total += count_value(i)
                if total > k:  # Early exit if the total exceeds k
                    return total
            return total

        # Binary search to find the maximum number
        left, right = 1, 2
        # Increase right bound exponentially until it potentially exceeds the required total value
        while total_value_up_to(right) <= k:
            right *= 2

        # Perform binary search
        while left < right:
            mid = (left + right) // 2
            if total_value_up_to(mid) <= k:
                left = mid + 1
            else:
                right = mid

        # Adjust the result to get the maximum number
        return left - 1


class Solution:
    def findMaximumNumber(self, k: int, x: int) -> int:
        # 初始化二分查找的边界
        left, right = 0, 10**18

        while left < right:
            mid = (left + right + 1) // 2  # 计算中间值
            bit_factor = 1  # 用于表示当前位的值（1, 2, 4, 8, ...）
            total_value = 0  # 累计满足条件的设置位的总数
            position = 1  # 位的位置（从1开始）

            # 遍历所有位，计算设置位的总数
            while bit_factor <= mid:
                if position % x == 0:
                    # 计算该位上的设置位数量，并累加到总数中
                    total_value += (mid // (2 * bit_factor)) * bit_factor
                    remaining = mid % (2 * bit_factor)
                    if remaining >= bit_factor:
                        total_value += remaining - bit_factor

                bit_factor *= 2
                position += 1

            # 根据设置位的总数调整二分查找的范围
            if total_value > k:
                right = mid - 1
            else:
                left = mid

        return left - 1  # 返回满足条件的最大数


s = Solution()
ans = s.findMaximumNumber(k=9, x=1)
print(ans)

s = Solution()
ans = s.findMaximumNumber(k=7, x=2)
print(ans)

s = Solution()
ans = s.findMaximumNumber(k=3278539330613, x=5)
print(ans)


class TrieNode:
    def __init__(self):
        self.is_word = False
        self.next = [None] * 26
        self.indices = []  # Storing indices where the word ends


class Trie:
    def __init__(self):
        self.root = TrieNode()

    def insert(self, word, index):
        now = self.root
        length = len(word)
        for i in range(length):
            pos = ord(word[i]) - ord('a')
            if now.next[pos] is None:
                now.next[pos] = TrieNode()
            now = now.next[pos]
        now.is_word = True
        now.indices.append(index)  # Store index at the end

    def search_indices(self, word):
        now = self.root
        n = len(word)
        for i in range(n):
            ch = ord(word[i]) - ord('a')
            if now.next[ch] != None:
                now = now.next[ch]
            else:
                return []
        return now.indices if now.is_word else []


class Solution:
    def beautifulIndices(self, s: str, a: str, b: str, k: int) -> List[int]:
        trie = Trie()
        for i in range(len(s) - len(a) + 1):
            trie.insert(s[i:i+len(a)], i)
        for i in range(len(s) - len(b) + 1):
            trie.insert(s[i:i+len(b)], i)

        indices_a = trie.search_indices(a)
        indices_b = trie.search_indices(b)

        beautiful_indices = []
        j = 0
        for i in indices_a:
            while j < len(indices_b) and indices_b[j] < i - k:
                j += 1
            if j < len(indices_b) and abs(indices_b[j] - i) <= k:
                beautiful_indices.append(i)

        return sorted(set(beautiful_indices))


class Solution:
    def beautifulIndices(self, s: str, a: str, b: str, k: int) -> List[int]:
        def find_occurrences(sub, string):
            start = 0
            while start < len(string):
                start = string.find(sub, start)
                if start == -1:
                    break
                yield start
                start += 1

        indices_a = list(find_occurrences(a, s))
        indices_b = list(find_occurrences(b, s))

        beautiful_indices = []
        j = 0
        for i in indices_a:
            while j < len(indices_b) and indices_b[j] < i - k:
                j += 1

            if j < len(indices_b) and abs(indices_b[j] - i) <= k:
                beautiful_indices.append(i)

        return sorted(set(beautiful_indices))


class Solution:
    def beautifulIndices(self, s: str, a: str, b: str, k: int) -> List[int]:
        # def find_occurrences(sub, string):
        #     for match in re.finditer(re.escape(sub), string):
        #         yield match.start()

        def compute_lps_array(sub):
            length = 0
            lps = [0] * len(sub)
            i = 1
            while i < len(sub):
                if sub[i] == sub[length]:
                    length += 1
                    lps[i] = length
                    i += 1
                else:
                    if length != 0:
                        length = lps[length - 1]
                    else:
                        lps[i] = 0
                        i += 1
            return lps

        def find_occurrences(sub, string):
            lps = compute_lps_array(sub)
            i, j = 0, 0
            while i < len(string):
                if sub[j] == string[i]:
                    i += 1
                    j += 1
                if j == len(sub):
                    yield i - j
                    j = lps[j - 1]
                elif i < len(string) and sub[j] != string[i]:
                    if j != 0:
                        j = lps[j - 1]
                    else:
                        i += 1

        indices_a = list(find_occurrences(a, s))
        indices_b = list(find_occurrences(b, s))

        beautiful_indices = set()

        for i in indices_a:
            lo, hi = 0, len(indices_b)
            while lo < hi:
                mid = (lo + hi) // 2
                if indices_b[mid] < i - k:
                    lo = mid + 1
                else:
                    hi = mid

            if lo < len(indices_b) and abs(indices_b[lo] - i) <= k:
                beautiful_indices.add(i)

        return sorted(beautiful_indices)


# # Example usage
# sol = Solution()
# s = "isawsquirrelnearmysquirrelhouseohmy"
# a = "my"
# b = "squirrel"
# k = 15
# ans = sol.beautifulIndices(s, a, b, k)
# print(ans)

# # Example 2
# s2 = "abcd"
# a2 = "a"
# b2 = "a"
# k2 = 4
# ans = sol.beautifulIndices(s2, a2, b2, k2)
# print(ans)
