from typing import List

# 输入：nums = [1,2,2,2]
# 输出：2
# 解释：我们将数组在下标 2 处分割，得到 [1,2,2] 和 [2] 。
# 数组 [1,2,2] 中，元素 2 是支配元素，因为它在数组中出现了 2 次，且 2 * 2 > 3 。
# 数组 [2] 中，元素 2 是支配元素，因为它在数组中出现了 1 次，且 1 * 2 > 1 。
# 两个数组 [1,2,2] 和 [2] 都有与 nums 一样的支配元素，所以这是一个合法分割。
# 下标 2 是合法分割中的最小下标。

# 示例 2：
# 输入：nums = [2,1,3,1,1,1,7,1,2,1]
# 输出：4
# 解释：我们将数组在下标 4 处分割，得到 [2,1,3,1,1] 和 [1,7,1,2,1] 。
# 数组 [2,1,3,1,1] 中，元素 1 是支配元素，因为它在数组中出现了 3 次，且 3 * 2 > 5 。
# 数组 [1,7,1,2,1] 中，元素 1 是支配元素，因为它在数组中出现了 3 次，且 3 * 2 > 5 。
# 两个数组 [2,1,3,1,1] 和 [1,7,1,2,1] 都有与 nums 一样的支配元素，所以这是一个合法分割。
# 下标 4 是所有合法分割中的最小下标。


# 示例 3：
# 输入：nums = [3,3,3,3,7,2,2]
# 输出：-1
# 解释：没有合法分割。
import collections

def minimumIndex(nums: List[int]) -> int:
    n = len(nums)

    count = collections.Counter(nums)
    dominator = max(count.keys(), key=count.get)

    prefix_sum = [0] * (n + 1)
    for i in range(n):
        prefix_sum[i + 1] = prefix_sum[i] + (nums[i] == dominator)

    print(prefix_sum)

    for i in range(1, n):
        if prefix_sum[i] > i / 2 and (prefix_sum[-1] - prefix_sum[i]) > (n - i) / 2:
            return i-1
    return -1




# 给你一个字符串 word 和一个字符串数组 forbidden 。
# 如果一个字符串不包含 forbidden 中的任何字符串，我们称这个字符串是 合法 的。
# 请你返回字符串 word 的一个 最长合法子字符串 的长度。
# 子字符串 指的是一个字符串中一段连续的字符，它可以为空。

# 在这个函数中，left 和 right 分别代表滑动窗口的左边界和右边界。当窗口内的字符串包含禁用的字符串时，
# 左边界向右移动，直到窗口内的字符串不再包含禁用的字符串。同时记录窗口的最大长度。
# 注意，这个函数只能处理所有禁用的字符串长度都不超过 word 的长度的情况。如果有禁用的字符串长度超过 word 的长度，可能需要对此函数进行修改。

# 这是一道动态规划问题，其思路是，首先初始化一个长度为 n 的数组 dp，其中 n 为输入字符串的长度。
# dp[i] 代表以第 i 个字符结尾的最长合法子字符串的长度。我们从左到右遍历字符串，并更新 dp 数组。
# 对于每个字符，我们都检查它与其前面的每个字符构成的子字符串是否合法（即是否不包含禁止的字符串）。
# 如果合法，我们更新 dp[i] 为这个子字符串的长度或 dp[i]，取两者的最大值。

# 然而，由于字符串的特性，上述的简单动态规划方法可能效率较低（时间复杂度为 O(n^2)）。
# 因此我们需要优化，其中一种优化的思路是，用一个哈希表存储字符串中每个字符的最后出现位置，
# 这样在检查子字符串是否合法时，我们就可以跳过不合法的部分，从而大大减少计算量。

# 在这个函数中，last_occurrence 是一个哈希表，记录了字符串中每个字符的最后出现位置。
# 对于每个字符，我们检查它与其前一个字符之间的子字符串是否合法。
# 如果不合法，我们更新该字符在 last_occurrence 中的值。
# 然后，我们更新 dp[i] 为 i - last_occurrence[word[i - 1]] 和 dp[i - 1] + 1 中的较小值，
# 这是因为最长合法子字符串的长度不能超过当前字符的位置与它前一个出现位置之间的距离。
# 同时，我们更新最大长度 max_len。最后，函数返回 max_len。


# 以下是一个用 Python 编写的动态规划解决方案的示例：

def check(s, forbidden):
    for forbid in forbidden:
        if forbid in s:
            return False
    return True

def longestValidSubstring(word, forbidden):
    n = len(word)
    dp = [0] * (n+1)
    
    for i in range(n+1):
        for j in range(i+1, n+1):
            if check(word[i:j], forbidden):
                dp[j] = max(dp[j], j-i)
            else:
                break
    return max(dp)

# print(longestValidSubstring(word = "cbaaaabc", forbidden = ["aaa","cb"]))
# print(longestValidSubstring(word = "leetcode", forbidden = ["de","le","e"]))


def maxLength(word, forbidden):
    forbidden_dict = {x: i for i in range(len(forbidden)) for x in forbidden[i]}
    n = len(word)
    begin = 0
    max_len = 0
    dp = [-1] * len(forbidden)
    for end in range(n):
        if word[end] in forbidden_dict:
            begin = max(begin, dp[forbidden_dict[word[end]]] + 1)
        max_len = max(max_len, end - begin + 1)
        if word[end] in forbidden_dict:
            dp[forbidden_dict[word[end]]] = end
    return max_len

# word = "cbaaaabc"
# forbidden = ["aaa","cb"]
# print(maxLength(word, forbidden))  # output: 4

# word = "leetcode"
# forbidden = ["de","le","e"]
# print(maxLength(word, forbidden))  # output: 4

class TrieNode:
    def __init__(self):
        self.children = {}
        self.is_end = False

class Solution:
    def __init__(self):
        self.root = TrieNode()

    def add_word(self, word):
        node = self.root
        for w in word:
            if w not in node.children:
                node.children[w] = TrieNode()
            node = node.children[w]
        node.is_end = True

    def search(self, word, index):
        node = self.root
        for i in range(index, -1, -1):
            if word[i] not in node.children:
                return False
            node = node.children[word[i]]
            if node.is_end:
                return True
        return False

    def longestValidSubstring(self, word, forbidden):
        for f in forbidden:
            self.add_word(f)
        n = len(word)
        dp = [0] * (n + 1)
        for i in range(1, n + 1):
            j = i - 1
            while j >= 0:
                if self.search(word[j:i], 0):
                    break
                dp[i] = max(dp[i], i - j)
                j -= 1
        return dp[-1]

s = Solution()
print(s.longestValidSubstring("cbaaaabc", ["aaa","cb"]))  # output: 4
print(s.longestValidSubstring("leetcode", ["de","le","e"]))  # output: 4


# 注意，这段代码中的cnt数组不仅仅是一个简单的计数器数组，它的每个元素存储的是在该位置或其左侧的数的总出现次数。
# 在计算过程中，我们对n-k的计数增加1，对n+k的计数减去1，这样在后续的累加过程中，可以得到正确的结果。这种方法被称为差分数组或前缀和数组的思想。
class Solution:
    def maximumBeauty(self, nums: List[int], k: int) -> int:
        # 找出nums中的最小和最大值
        n_min = min(nums)
        n_max = max(nums)
        
        # 计算经过+k和-k操作后的数的范围
        l_min = n_min - k
        l_max = n_max + k
        range_size = l_max - l_min + 1  # 计算数的范围大小

        # 创建一个计数器数组，用于记录范围内每个数的出现次数
        cnt = [0] * range_size

        # 遍历nums中的每个数，分别计算其+k和-k后的数，更新计数器数组
        for n in nums:
            cnt[n - k - l_min] += 1  # n-k对应的计数增加1
            # n+k对应的计数减去1，因为在之后的计数累计过程中，
            # 我们需要减去对n+k的计数，以避免将n+k的计数算入到n中
            if n + k + 1 - l_min < range_size:
                cnt[n + k + 1 - l_min] -= 1

        # 将计数器数组中的数进行累加，以得到在该位置或其左侧的数的总出现次数
        for i in range(1, range_size):
            cnt[i] += cnt[i - 1]

        # 返回计数器数组中的最大值，即为nums中可以通过+k或-k得到的数的最大出现次数
        return max(cnt)


class Solution:
    def maximumBeauty(self, nums: List[int], k: int) -> int:
        # 找出nums中的最小和最大值
        n_min = min(nums)
        n_max = max(nums)
        
        # 计算经过+k和-k操作后的数的范围
        l_min = n_min - k
        l_max = n_max + k
        range_size = l_max - l_min + 1  # 计算数的范围大小

        # 创建一个计数器数组，用于记录范围内每个数的出现次数
        cnt = [0] * range_size

        # 遍历nums中的每个数，分别计算其+k和-k后的数，更新计数器数组
        for n in nums:
            cnt[n - k - l_min] += 1  # n-k对应的计数增加1
            # n+k对应的计数减去1，因为在之后的计数累计过程中，
            # 我们需要减去对n+k的计数，以避免将n+k的计数算入到n中
            if n + k + 1 - l_min < range_size:
                cnt[n + k + 1 - l_min] -= 1

        # 将计数器数组中的数进行累加，以得到在该位置或其左侧的数的总出现次数
        for i in range(1, range_size):
            cnt[i] += cnt[i - 1]

        # 返回计数器数组中的最大值，即为nums中可以通过+k或-k得到的数的最大出现次数
        return max(cnt)
