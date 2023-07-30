def countPerfectSubarrays(nums):
    from collections import defaultdict
    nums_count = defaultdict(int)  # 用于存储数组中每个元素的数量
    window_count = defaultdict(int)  # 用于存储窗口中每个元素的数量
    for num in nums:
        nums_count[num] += 1
        
    perfect_count = 0  # 完全子数组的数量
    left, right = 0, 0  # 定义滑动窗口的左右边界
    
    while right < len(nums):
        window_count[nums[right]] += 1  # 将右边界的元素添加到窗口中
        while len(nums_count) == len(window_count):  # 如果窗口是完全子数组
            print(left, right)
            perfect_count += len(nums) - right  # 计算满足条件的子数组数量
            window_count[nums[left]] -= 1  # 移除左边界的元素
            if window_count[nums[left]] == 0:  # 如果某个元素的数量变为0，删除这个元素
                del window_count[nums[left]]
            left += 1  # 左边界右移
        right += 1  # 右边界右移
    return perfect_count


# 这个问题可以通过枚举所有可能的组合来解决。我们需要找到所有可能的字符串，
# 这些字符串是由输入字符串 a、b 和 c 通过一定的顺序组合并可能的插入其他字符得到的。
# 然后我们选择长度最短的字符串，如果长度相同，则选择字典序最小的字符串。

# 具体步骤如下：
# 首先，我们需要定义一个函数 merge(s1, s2) 来合并两个字符串。
# 该函数找到 s1 和 s2 的最长公共后缀和前缀，然后返回合并的结果。例如 merge("abc", "bcd") 返回 "abcd"。

# 然后，我们对三个字符串的所有可能的顺序进行枚举。
# 总共有 6 种可能的顺序，即 (a, b, c)，(a, c, b)，(b, a, c)，(b, c, a)，(c, a, b)，(c, b, a)。

# 对于每一种顺序，我们使用 merge 函数来合并这三个字符串，并保存结果。
# 最后，我们找出所有结果中长度最短的字符串。
# 如果长度相同，则选择字典序最小的字符串。

# 这个方法的时间复杂度是 O(n^2)，其中 n 是输入字符串的最大长度
# 。因为我们需要枚举所有可能的顺序，并且 merge 函数的时间复杂度是 O(n^2)。
# 但是由于输入字符串的长度通常不会太长，所以这个方法在实践中是可行的。

from itertools import permutations

def merge(s1, s2):
    # Special case: if one string is a substring of the other
    if s1 in s2:
        return s2
    if s2 in s1:
        return s1
    # Find the longest common suffix of s1 and prefix of s2
    for i in range(len(s1) + 1):
        if s2.startswith(s1[i:]):
            return s1[:i] + s2
    return s1 + s2

def shortest_supersequence(a, b, c):
    ans = None
    # Enumerate all possible orders
    for s1, s2, s3 in permutations((a, b, c)):
        # Merge the strings in the current order
        cur = merge(merge(s1, s2), s3)
        # If this is the first result or it's better than the current best result
        if ans is None or len(cur) < len(ans) or (len(cur) == len(ans) and cur < ans):
            ans = cur
    return ans

# # Test the function with the provided examples
# print(shortest_supersequence("abc", "bca", "aaa"))  # Output: "aaabca"
# print(shortest_supersequence("ab", "ba", "aba"))  # Output: "aba"
# print(shortest_supersequence("cab", "a", "b"))  # Output: "aba"

class Solution:
    def countSteppingNumbers(self, low: str, high: str) -> int:
        MOD = 10**9 + 7
        prefix = [[0]*10 for _ in range(11)]
        dp = [[0]*10 for _ in range(11)]
        for i in range(10):
            dp[1][i] = 1
        for i in range(1, 10):
            prefix[1][i] = prefix[1][i-1] + dp[1][i]
        for i in range(2, 11):
            for j in range(10):
                if j > 0:
                    dp[i][j] += dp[i-1][j-1]
                if j < 9:
                    dp[i][j] += dp[i-1][j+1]
                dp[i][j] %= MOD
                prefix[i][j] = prefix[i][j-1] + dp[i][j]
                prefix[i][j] %= MOD

        def calc(x):
            if x == 0:
                return 0
            s = str(x)
            n = len(s)
            last_digit = int(s[0])
            res = 0
            for i in range(1, n):
                res += prefix[i][9]
            for i in range(1, last_digit):
                res += dp[n][i]
            for i in range(1, n):
                curr_digit = int(s[i])
                if abs(last_digit-curr_digit) != 1:
                    break
                for j in range(last_digit):
                    res += dp[n-i][j]
                last_digit = curr_digit
                if i == n-1:
                    res += 1
            res %= MOD
            return res

        return (calc(int(high)) - calc(int(low) - 1)) % MOD  # 减1是因为我们需要包含low本身

# 测试一下
sol = Solution()
print(sol.countSteppingNumbers("20", "21"))  # 输出应该是1
