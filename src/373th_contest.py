
from typing import List

# 给你一个大小为 m x n 的整数矩阵 mat 和一个整数 k 。
# 请你将矩阵中的 奇数 行循环 右 移 k 次，偶数 行循环 左 移 k 次。
# 如果初始矩阵和最终矩阵完全相同，则返回 true ，否则返回 false 。

class Solution:
    def areSimilar(self, mat: List[List[int]], k: int) -> bool:
        m, n = len(mat), len(mat[0])
        shifted_mat = []

        for i in range(m):
            if (i + 1) % 2 == 1:  
                shifted_row = mat[i][-k % n:] + mat[i][:-k % n]
            else:  
                shifted_row = mat[i][k % n:] + mat[i][:k % n]
            shifted_mat.append(shifted_row)

        return shifted_mat == mat

mat = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
]
# k = 1
# s = Solution()
# ans = s.areSimilar(mat = [[1,2]], k = 1)
# print(ans)

# 1 <= s.length <= 1000
# 1 <= k <= 1000
# s 仅由小写英文字母组成。


# 给你一个字符串 s 和一个正整数 k 。
# 用 vowels 和 consonants 分别表示字符串中元音字母和辅音字母的数量。
# 如果某个字符串满足以下条件，则称其为 美丽字符串 ：
#   vowels == consonants，即元音字母和辅音字母的数量相等。
#   (vowels * consonants) % k == 0，即元音字母和辅音字母的数量的乘积能被 k 整除。
# 返回字符串 s 中 非空美丽子字符串 的数量。
# 子字符串是字符串中的一个连续字符序列。

# 英语中的 元音字母 为 'a'、'e'、'i'、'o' 和 'u' 。
# 英语中的 辅音字母 为除了元音字母之外的所有字母。

class Solution:
    def beautifulSubstrings(self, s: str, k: int) -> int:
        vowels = set("aeiou")
        n = len(s)
        beautiful_count = 0

        prefix_vowels = [0] * (n + 1)
        prefix_consonants = [0] * (n + 1)

        for i in range(1, n + 1):
            prefix_vowels[i] = prefix_vowels[i - 1] + (1 if s[i - 1] in vowels else 0)
            prefix_consonants[i] = prefix_consonants[i - 1] + (1 if s[i - 1] not in vowels else 0)

        for i in range(n):
            for j in range(i + 1, n + 1):
                v_count = prefix_vowels[j] - prefix_vowels[i]
                c_count = prefix_consonants[j] - prefix_consonants[i]
                if v_count == c_count and (v_count * c_count) % k == 0:
                    beautiful_count += 1

        return beautiful_count


# # Example usage

# s = Solution()
# ans = s.beautifulSubstrings(s = "baeyh", k = 2)
# print(ans)
        
# ans = s.beautifulSubstrings(s = "abba", k = 1)
# print(ans)        

# ans = s.beautifulSubstrings(s = "bcdf", k = 1)
# print(ans)    


# 给你一个下标从 0 开始的 正整数 数组 nums 和一个 正整数 limit 。
# 在一次操作中，你可以选择任意两个下标 i 和 j，如果 满足 |nums[i] - nums[j]| <= limit ，则交换 nums[i] 和 nums[j] 。
# 返回执行任意次操作后能得到的 字典序最小的数组 。

# 如果在数组 a 和数组 b 第一个不同的位置上，
# 数组 a 中的对应字符比数组 b 中的对应字符的字典序更小，则认为数组 a 就比数组 b 字典序更小。
# 例如，数组 [2,10,3] 比数组 [10,2,3] 字典序更小，下标 0 处是两个数组第一个不同的位置，且 2 < 10 。
class Solution:
    def lexicographicallySmallestArray(self, nums: List[int], limit: int) -> List[int]:
        n = len(nums)
        
        sorted_nums = sorted((num, i) for i, num in enumerate(nums))

        groups = []
        current_group = []
        for i in range(n):
            if i == 0 or sorted_nums[i][0] - sorted_nums[i - 1][0] <= limit:
                current_group.append(sorted_nums[i])
            else:
                groups.append(current_group)
                current_group = [sorted_nums[i]]
        groups.append(current_group)

        result = [None] * n
        for group in groups:
            dix = [x[1] for x in group]
            dix.sort()  
            for i in range(len(dix)):
                result[dix[i]] = group[i][0]

        return result

# # Test the function with the provided examples
# example1 = [1,5,3,9,8]
# limit1 = 2
# example2 = [1,7,6,18,2,1]
# limit2 = 3
# example3 = [1,7,28,19,10]
# limit3 = 3


# s = Solution()
# ans = s.lexicographicallySmallestArray(example1, limit1)
# print(ans)

# s = Solution()
# ans = s.lexicographicallySmallestArray(example2, limit2)
# print(ans)

# s = Solution()
# ans = s.lexicographicallySmallestArray(example3, limit3)
# print(ans)

# s = Solution()
# ans = s.lexicographicallySmallestArray([1,60,34,84,62,56,39,76,49,38], 4)
# print(ans) # 1,56,34,84,60,62,38,76,49,39]



class SegmentTreeNode:
    def __init__(self, vowel_count=0, consonant_count=0):
        self.vowel_count = vowel_count
        self.consonant_count = consonant_count

class SegmentTree:
    def __init__(self, data):
        self.n = len(data)
        self.tree = [SegmentTreeNode() for _ in range(4 * self.n)]
        self.build_tree(data, 0, 0, self.n - 1)

    def build_tree(self, data, node, start, end):
        if start == end:
            # Leaf node will have a single element
            if data[start] in 'aeiou':
                self.tree[node].vowel_count = 1
            else:
                self.tree[node].consonant_count = 1
        else:
            mid = (start + end) // 2
            # Recursively build the Segment tree
            self.build_tree(data, 2 * node + 1, start, mid)
            self.build_tree(data, 2 * node + 2, mid + 1, end)
            # Internal node will have the sum of both of its children
            self.tree[node].vowel_count = self.tree[2 * node + 1].vowel_count + self.tree[2 * node + 2].vowel_count
            self.tree[node].consonant_count = self.tree[2 * node + 1].consonant_count + self.tree[2 * node + 2].consonant_count

    def query(self, node, start, end, l, r):
        if r < start or end < l:
            # range represented by a node is completely outside the given range
            return SegmentTreeNode()
        if l <= start and end <= r:
            # range represented by a node is completely inside the given range
            return self.tree[node]
        # range represented by a node is partially inside and partially outside the given range
        mid = (start + end) // 2
        left_query = self.query(2 * node + 1, start, mid, l, r)
        right_query = self.query(2 * node + 2, mid + 1, end, l, r)
        # Combine the results of left and right children
        return SegmentTreeNode(left_query.vowel_count + right_query.vowel_count,
                               left_query.consonant_count + right_query.consonant_count)
        
class Solution:
    def beautifulSubstrings(self, s: str, k: int) -> int:
        n = len(s)
        segment_tree = SegmentTree(s)
        count = 0

        for start in range(n):
            for end in range(start, n):
                query_result = segment_tree.query(0, 0, n - 1, start, end)
                vowels = query_result.vowel_count
                consonants = query_result.consonant_count

                if vowels == consonants and (vowels * consonants) % k == 0:
                    count += 1

        return count

# Example usage

s = Solution()
ans = s.beautifulSubstrings(s = "baeyh", k = 2)
print(ans) # 2
        
ans = s.beautifulSubstrings(s = "abba", k = 1)
print(ans) # 3       

ans = s.beautifulSubstrings(s = "bcdf", k = 1)
print(ans) # 0   
