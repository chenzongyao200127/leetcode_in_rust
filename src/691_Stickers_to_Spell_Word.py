# 691_Stickers_to_Spell_Word
# https://leetcode.cn/problems/stickers-to-spell-word/

# 我们有 n 种不同的贴纸。每个贴纸上都有一个小写的英文单词。
# 您想要拼写出给定的字符串 target ，方法是从收集的贴纸中切割单个字母并重新排列它们。
# 如果你愿意，你可以多次使用每个贴纸，每个贴纸的数量是无限的。
# 返回你需要拼出 target 的最小贴纸数量。如果任务不可能，则返回 -1 。
# 注意：在所有的测试用例中，所有的单词都是从 1000 个最常见的美国英语单词中随机选择的，并且 target 被选择为两个随机单词的连接。


# 示例 1：
# 输入： stickers = ["with","example","science"], target = "thehat"
# 输出：3
# 解释：
# 我们可以使用 2 个 "with" 贴纸，和 1 个 "example" 贴纸。
# 把贴纸上的字母剪下来并重新排列后，就可以形成目标 “thehat“ 了。
# 此外，这是形成目标字符串所需的最小贴纸数量。

# 示例 2:
# 输入：stickers = ["notice","possible"], target = "basicbasic"
# 输出：-1
# 解释：我们不能通过剪切给定贴纸的字母来形成目标“basicbasic”。

# 提示:
# n == stickers.length
# 1 <= n <= 50
# 1 <= stickers[i].length <= 10
# 1 <= target.length <= 15
# stickers[i] 和 target 由小写英文单词组成


# We want to perform an exhaustive search, but we need to speed it up based on the input data 
# being random. For all stickers, we can ignore any letters that are not in the target word. 
# When our candidate answer won't be smaller than an answer we have already found, 
# we can stop searching this path. When a sticker dominates another, 
# we shouldn't include the dominated sticker in our sticker collection.
# [Here, we say a sticker `A` dominates `B` if `A.count(letter) >= B.count(letter)` for all letters.]

from typing import List
from collections import defaultdict

# 超时
class Solution:
    def minStickers(self, stickers: List[str], target: str) -> int:
        # 计算单词中每个字符的频率
        def count_word(word):
            count = defaultdict(int)
            for ch in word:
                count[ch] += 1
            return count

        # 使用每张贴纸更新当前目标
        def use_sticker(target_count, sticker_count):
            new_target = target_count.copy()
            for ch, freq in sticker_count.items():
                new_target[ch] -= min(new_target[ch], freq)
            return new_target
        
        # 记忆化递归搜索
        def search(target_count):
            # 如果此状态已计算过，直接返回
            if tuple(target_count.items()) in memo:
                return memo[tuple(target_count.items())]
            
            # 初始设置结果为无穷大
            ans = float('inf')

            # 如果目标字母都已满足，返回 0
            if all(val == 0 for val in target_count.values()):
                return 0
            
            # 尝试每个贴纸
            for sticker_count in stickers_counts:
                # 如果贴纸中的任何一个字母存在于目标中并且仍未满足，则考虑使用此贴纸
                if any(ch in target_count and target_count[ch] > 0 for ch in sticker_count.keys()):
                    new_target = use_sticker(target_count, sticker_count)
                    # 递归地搜索使用此贴纸后的最小数量
                    temp_ans = search(new_target)
                    if temp_ans != -1:
                        ans = min(ans, 1 + temp_ans)
            
            # 如果答案仍然为无穷大，返回 -1
            memo[tuple(target_count.items())] = -1 if ans == float('inf') else ans
            return memo[tuple(target_count.items())]
            
        target_count = count_word(target)
        # 只取有用的贴纸，即那些至少有一个字母存在于目标中的贴纸
        stickers_counts = [count_word(sticker) for sticker in stickers if set(sticker) & set(target)]
        memo = {}
        
        res = search(target_count)
        
        # 如果结果为无穷大，返回 -1
        return -1 if res == float('inf') else res

# s = Solution()
# ans = s.minStickers(stickers = ["with","example","science"], target = "thehat")
# print(ans)

# ans = s.minStickers(stickers = ["notice","possible"], target = "basicbasic")
# print(ans)


# 状态压缩DP

# 使用状态压缩DP优化是一个好的方法，特别是在处理此类问题时，其中每个字母都可以在贴纸中出现或不出现。
# 以下是如何使用状态压缩DP来解决这个问题的大致步骤：
# 1. **状态表示**：对于目标字符串`target`，我们可以使用一个整数来表示它的某个子集，
# 其中`target`的第i个字符是否存在可以由该整数的第i个位来表示。
# 例如，如果`target = "abc"`，则整数`5 (101 in binary)`表示子集`"ac"`。

# 2. **状态转移**：对于上面的每个子集，我们需要知道形成它所需的最小贴纸数量。
# 我们可以遍历每个贴纸，并尝试使用它来减少目标子集的大小。
# 如果使用贴纸后，目标子集变为另一个子集，我们就有了一个可能的状态转移。

# 3. **边界条件**：如果子集已经为空（即该整数为0），则所需的最小贴纸数量为0。

# 以下是状态压缩DP的Python代码：
def minStickers(stickers, target):
    n = len(target)
    m = 1 << n  # 使用位操作计算2的n次方，这是因为target的每个字符都可以在或不在某个子集中，总共有2^n种可能的子集。
    dp = [float('inf')] * m  # 初始化dp数组，其中每个值表示形成对应子集所需的最小贴纸数量。
    dp[0] = 0  # 如果子集为空（没有字符），则不需要任何贴纸。

    # 遍历所有可能的状态
    for i in range(m):
        # 如果当前状态无法从之前的状态转移到，那么它的值会是无穷大，我们可以跳过它。
        if dp[i] == float('inf'):
            continue
        
        # 尝试应用每一个贴纸，看看它是否能帮助我们形成新的子集。
        for sticker in stickers:
            # 初始化下一个状态为当前状态
            nxt = i 
            # 遍历贴纸中的每个字符
            for ch in sticker:
                # 遍历目标字符串中的每个字符
                for k in range(n):
                    # 检查贴纸中的字符是否与目标字符串中的字符匹配，并且这个字符在当前状态中还没有被覆盖。
                    if target[k] == ch and not (nxt & (1 << k)):
                        nxt |= 1 << k  # 使用位操作更新状态，表示这个字符现在已经被覆盖。
                        print(nxt)
                        break          # 跳出循环，因为我们每次只能使用一个贴纸中的字符一次。
                    
            # 更新dp数组的值，如果通过当前贴纸我们能得到一个更小的数量，那么就更新它。
            dp[nxt] = min(dp[nxt], dp[i] + 1)

    # 如果最后一个状态（所有字符都被覆盖）的值仍然是无穷大，那么返回-1，表示无法形成目标字符串。
    # 否则，返回这个值，它表示形成目标字符串所需的最小贴纸数量。
    return dp[-1] if dp[-1] != float('inf') else -1


ans = minStickers(stickers = ["with","example","science"], target = "thehat")
print(ans)

ans = minStickers(stickers = ["notice","possible"], target = "basicbasic")
print(ans)

ans = minStickers(["summer","sky","cent","bright","kill","forest","neighbor","capital","tall"], "originalchair")
print(ans)

# 这个方法的时间复杂度是\(O(2^n \times len(stickers) \times len(target))\)，其中n是`target`的长度。

# 状态压缩是一种动态规划的优化方法，它的核心思想是用位操作来表示和处理状态，
# 从而减少时间和空间的复杂度。状态压缩在处理组合状态问题时特别有效，例如子集、排列或序列问题。
# 以下是使用状态压缩优化动态规划的一些关键点：

# 1. **位表示**：状态压缩通常使用整数的二进制位来表示状态。
# 例如，对于一个长度为 \(n\) 的集合，它的每一个子集都可以用一个长度为 \(n\) 的二进制数来表示。
# 在这个二进制数中，1 表示该位置的元素存在于子集中，而 0 表示不存在。

# 2. **空间效率**：传统的动态规划可能需要多维数组来存储状态，
# 而状态压缩通过将这些维度压缩为一个整数来优化存储需求。

# 3. **快速转移**：位操作（如 &、| 和 ^）可以用于快速处理和转移状态，
# 而无需复杂的循环或递归。

# 4. **子集遍历**：在某些情况下，你可能需要遍历一个集合的所有子集。
# 使用状态压缩和位操作，你可以非常高效地完成这个任务。

# 5. **直观性**：尽管状态压缩初看起来可能很复杂，
# 但它实际上提供了一种非常直观的方式来处理和表示组合状态问题。

# 在本质上，状态压缩利用了二进制表示的高效性和位操作的速度，
# 将多维或组合状态问题转化为一维问题，从而优化动态规划的性能。
# 但是，它也要求你对位操作有深入的理解，并且能够灵活地应用它们来处理复杂的状态转移。