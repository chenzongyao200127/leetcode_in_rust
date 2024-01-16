# 2719_统计整数数目
# https://leetcode.cn/problems/count-of-integers/description/

# 给你两个数字字符串 num1 和 num2 ，以及两个整数 max_sum 和 min_sum 。
# 如果一个整数 x 满足以下条件，我们称它是一个好整数：

# num1 <= x <= num2
# min_sum <= digit_sum(x) <= max_sum.
# 请你返回好整数的数目。答案可能很大，请返回答案对 109 + 7 取余后的结果。

# 注意，digit_sum(x) 表示 x 各位数字之和。

from functools import cache


class Solution:
    def count(self, num1: str, num2: str, min_sum: int, max_sum: int) -> int:
        # 计算小于等于给定数字的合法数的数量
        def count_valid_numbers(upper_bound: str) -> int:
            @cache
            def dfs(index: int, current_sum: int, is_limit: bool) -> int:
                # 超过最大和的情况不合法
                if current_sum > max_sum:
                    return 0

                # 达到数字长度时，检查和是否在合法范围内
                if index == len(upper_bound):
                    return current_sum >= min_sum

                res = 0
                max_digit = int(upper_bound[index]) if is_limit else 9
                for digit in range(max_digit + 1):
                    res += dfs(index + 1, current_sum + digit,
                               is_limit and digit == max_digit)
                return res

            return dfs(0, 0, True)

        # 检查 num1 是否为合法数字
        is_num1_valid = min_sum <= sum(map(int, num1)) <= max_sum
        # 计算 num2 和 num1 之间合法数字的数量，并根据 num1 的合法性进行调整
        return (count_valid_numbers(num2) - count_valid_numbers(num1) + is_num1_valid) % 1_000_000_007
