# 1215_步进数
# https://leetcode.cn/problems/stepping-numbers/description/


# 如果一个整数上的每一位数字与其相邻位上的数字的绝对差都是 1，那么这个数就是一个「步进数」。
# 例如，321 是一个步进数，而 421 不是。
# 给你两个整数，low 和 high，请你找出在 [low, high] 范围内的所有步进数，并返回 排序后 的结果。


from typing import List


class Solution:
    def countSteppingNumbers(self, low: int, high: int) -> List[int]:
        if low == 0:
            result = [0]
        else:
            result = []

        queue = [i for i in range(1, 10)]

        while queue:
            current = queue.pop(0)
            if current <= high:
                if current >= low:
                    result.append(current)
                last_digit = current % 10
                if last_digit > 0:
                    queue.append(current * 10 + last_digit - 1)
                if last_digit < 9:
                    queue.append(current * 10 + last_digit + 1)

        return result


# 示例
sol = Solution()
print(sol.countSteppingNumbers(0, 21))  # 输出应该是 [0,1,2,3,4,5,6,7,8,9,10,12,21]
