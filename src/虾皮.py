
#
# Note: 类名、方法名、参数名已经指定，请勿修改
#
#
#
# @param s string字符串
# @param t string字符串
# @return string字符串
#
import re
from collections import Counter, defaultdict
import re
from collections import Counter, defaultdict

# 10 单选 + 6 多选 + 3 编程 难度好大.. （编程占55分）
# 单选和多选比较杂，大部分题与安全相关还有部分基础题
# 编程题
# 1. 给定字符串 s 和 t，在s中找出包括所有t字符的第一个最小长度的子串 （hashmap + 滑动窗口）
# 2. 给定一个正整数n，其值取值范围会很大， 计算 0到n（包括n）之间包含2的数字出现的个数 (数位dp, 不会写..应该是原题，得背模板，过了28% ，太难了)
# 3. 提取一个字符串中的所有ipv4，并用逗号间隔输出（应该是考察正则表达式...寄 0%，其中有 255.255.255.0.0这种例子，不知道有没有内置库）
# 太难了，没针对准备过数位DP和真正表达式，完全不会，1个小时就交卷了

from collections import Counter, defaultdict


class Solution:
    def find_min_str(self, s: str, t: str) -> str:
        """
        Finds the minimum window substring in `s` which contains all the characters of `t`.

        Args:
        s (str): The main string where we need to find the window.
        t (str): The target string containing characters to be covered in the window.

        Returns:
        str: The smallest substring of `s` that contains all the characters of `t`.
             Returns an empty string if no such window exists.
        """
        if not s or not t:
            return ""

        need = Counter(t)
        window = defaultdict(int)
        left = right = 0
        valid = 0
        min_len = float('inf')
        min_start = 0

        while right < len(s):
            c = s[right]
            right += 1
            if c in need:
                window[c] += 1
                if window[c] == need[c]:
                    valid += 1

            while valid == len(need):
                if right - left < min_len:
                    min_len = right - left
                    min_start = left

                d = s[left]
                left += 1
                if d in need:
                    if window[d] == need[d]:
                        valid -= 1
                    window[d] -= 1

        return s[min_start:min_start + min_len] if min_len != float('inf') else ""


# Example usage
sol = Solution()
print(sol.find_min_str("ADOBECODEBANC", "ABC"))  # Output: "BANC"


#
# Note: 类名、方法名、参数名已经指定，请勿修改
#
#
# 返回到0到n之间数字2出现的次数
# @param n int整型
# @return int整型
#
class Solution:
    def countOfTwos(self, n):
        # write code here
        s = str(n)

        length = len(s)

        dp = [[[-1 for _ in range(2)] for _ in range(2)]
              for _ in range(length)]
        # print(dp)

        def dfs(pos, f_two, tight):
            if pos == length:
                return 0

            if dp[pos][f_two][tight] != -1:
                return dp[pos][f_two][tight]

            limit = int(s[pos]) if tight else 9
            res = 0
            for d in range(0, limit + 1):
                d_two = (1 if d == 2 else 0)
                res += dfs(pos + 1, f_two or d == 2, tight and d ==
                           limit) + (d_two if f_two or d == 2 else 0)

            dp[pos][f_two][tight] = res
            return res

        return dfs(0, 0, 1)


def extract_ipv4_addresses(text):
    ipv4_pattern = r'\b(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b'
    addresses = re.findall(ipv4_pattern, text)
    result = ", ".join(addresses)
    return result
