# 2376. 统计特殊整数
# https://leetcode.cn/problems/count-special-integers/description/


# 如果一个正整数每一个数位都是 互不相同 的，我们称它是 特殊整数 。
# 给你一个 正 整数 n ，请你返回区间 [1, n] 之间特殊整数的数目。


class Solution:
    def countSpecialNumbers(self, n: int) -> int:
        number_str = str(n)

        @cache
        def dp(index: int, used_digits_mask: int, is_within_limit: bool, has_started: bool) -> int:
            """ 
            数位DP函数。
            index: 当前处理的数位索引。
            used_digits_mask: 已使用数字的位掩码。
            is_within_limit: 当前是否受n的限制。
            has_started: 是否已经开始选择数字（用于处理前导零）。
            """
            if index == len(number_str):
                return int(has_started)  # 只有开始选择数字后才算一个合法的数字

            total_count = 0
            if not has_started:  # 考虑跳过当前数位，不选择任何数字
                total_count = dp(index + 1, used_digits_mask, False, False)

            start_digit = 1 if not has_started else 0  # 处理前导零的情况
            end_digit = int(number_str[index]) if is_within_limit else 9

            for digit in range(start_digit, end_digit + 1):
                if (used_digits_mask >> digit & 1) == 0:  # 检查数字是否未被使用
                    new_mask = used_digits_mask | (1 << digit)
                    still_within_limit = is_within_limit and digit == end_digit
                    total_count += dp(index + 1, new_mask,
                                      still_within_limit, True)

            return total_count

        return dp(0, 0, True, False)


