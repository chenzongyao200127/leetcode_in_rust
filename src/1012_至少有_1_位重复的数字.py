# 1012_至少有_1_位重复的数字
# https://leetcode.cn/problems/numbers-with-repeated-digits/description/

# 给定正整数 n，返回在 [1, n] 范围内具有 至少 1 位 重复数字的正整数的个数。

# 示例 1：
# 输入：n = 20
# 输出：1
# 解释：具有至少 1 位重复数字的正数（<= 20）只有 11 。

# 示例 2：
# 输入：n = 100
# 输出：10
# 解释：具有至少 1 位重复数字的正数（<= 100）有 11，22，33，44，55，66，77，88，99 和 100 。

# 示例 3：
# 输入：n = 1000
# 输出：262
from functools import lru_cache

# 数位DP 模板
class Solution:
    def numDupDigitsAtMostN(self, n: int) -> int:
        n_str = str(n)

        # 记忆化搜索
        @lru_cache(None)
        def count_numbers_without_repeated_digits(index: int, used_digits_mask: int, is_limit: bool, has_started: bool) -> int:
            # If we have processed all digits
            if index == len(n_str):
                return int(has_started)

            total_count = 0

            # If the current digit hasn't started, we can skip the current position.
            if not has_started:
                total_count = count_numbers_without_repeated_digits(index + 1, used_digits_mask, False, False)

            # Determine the range of possible digits at the current position.
            start_digit = 1 if not has_started else 0
            end_digit = int(n_str[index]) if is_limit else 9

            # Enumerate possible digits for the current position.
            for digit in range(start_digit, end_digit + 1):
                if not (used_digits_mask >> digit & 1):  # If the digit hasn't been used before.
                    total_count += count_numbers_without_repeated_digits(index + 1, used_digits_mask | (1 << digit), is_limit and digit == end_digit, True)

            return total_count

        return n - count_numbers_without_repeated_digits(0, 0, True, False)


s = Solution()
ans = s.numDupDigitsAtMostN(2000)
print(ans)


# 官解
class Solution:
    def numDupDigitsAtMostN(self, n: int) -> int:
        # 将整数 n 转为数字列表，例如 n = 1234 则 A = [1, 2, 3, 4]
        A = list(map(int, str(n)))
        
        # 数字 n 的长度
        N = len(A)

        # f 是一个记忆化搜索的函数
        # i 表示当前正在处理第i位（从0开始，从高到低）
        # tight 表示当前的数是否受到限制（即之前的每一位都等于对应的数字 n 的数位）
        # mask 表示已经使用过的数字（二进制表示），例如如果使用过数字 2 和 3，那么 mask = ...001100
        # hasDup 表示当前数是否已经有重复的数字
        @lru_cache(None)
        def f(i, tight, mask, hasDup):
            # 如果已经处理完 n 的所有数位
            if i >= N:
                # 如果有重复的数字，返回1，否则返回0
                return 1 if hasDup else 0
            
            # 如果受到限制，上限是当前数位，否则上限是9
            upperLimit = A[i] if tight else 9
            ans = 0

            # 对于当前数位，从0遍历到upperLimit
            for d in range(upperLimit + 1):
                # 如果 d 等于 upperLimit，则 tight2 为 True
                tight2 = tight and d == upperLimit
                # 更新 mask。如果 mask 为 0 且 d 为 0，则 mask 不变。否则，将 d 加入 mask
                mask2 = mask if mask == 0 and d == 0 else mask | (1 << d)
                # 如果 mask 中已经有了数字 d，则 hasDup2 为 True
                hasDup2 = hasDup or (mask & (1 << d))
                # 累加答案
                ans += f(i + 1, tight2, mask2, hasDup2)

            return ans
        
        # 从第0位、受限制、没有使用过任何数字、没有重复数字开始搜索
        return f(0, True, 0, False)
    
# 0X3F 题解
class Solution:
    def numDupDigitsAtMostN(self, n: int) -> int:
        s = str(n)
        
        @lru_cache(None)
        def f(i: int, mask: int, is_limit: bool, is_num: bool) -> int:
            if i == len(s):
                return int(is_num)  # is_num 为 True 表示得到了一个合法数字
            res = 0
            if not is_num:  # 可以跳过当前数位
                res = f(i + 1, mask, False, False)
            low = 0 if is_num else 1  # 如果前面没有填数字，必须从 1 开始（因为不能有前导零）
            up = int(s[i]) if is_limit else 9  # 如果前面填的数字都和 n 的一样，那么这一位至多填 s[i]（否则就超过 n 啦）
            for d in range(low, up + 1):  # 枚举要填入的数字 d
                if (mask >> d & 1) == 0:  # d 不在 mask 中
                    res += f(i + 1, mask | (1 << d), is_limit and d == up, True)
            return res
        
        return n - f(0, 0, True, False)
