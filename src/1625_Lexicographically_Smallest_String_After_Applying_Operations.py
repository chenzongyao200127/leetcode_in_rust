# // 1625_Lexicographically_Smallest_String_After_Applying_Operations
# // https://leetcode.cn/problems/lexicographically-smallest-string-after-applying-operations/

# https://leetcode.cn/problems/lexicographically-smallest-string-after-applying-operations/solution/python3javacgo-yi-ti-shuang-jie-bfs-bao-xl8n2/
# 我们观察发现，对于累加操作，数字最多累加 10 次，就会回到原来的状态；对于轮转操作，字符串最多轮转 n 次，也会回到原来的状态。
# 所以，我们直接枚举所有的字符串状态，取字典序最小的状态即可。
class Solution:
    def findLexSmallestString(self, s: str, a: int, b: int) -> str:
        ans = s
        n = len(s)
        s = list(s)
        for _ in range(n):
            s = s[-b:] + s[:-b]
            for j in range(10):
                for k in range(1, n, 2):
                    s[k] = str((int(s[k]) + a) % 10)
                if b & 1:
                    for p in range(10):
                        for k in range(0, n, 2):
                            s[k] = str((int(s[k]) + a) % 10)
                        t = ''.join(s)
                        if ans > t:
                            ans = t
                else:
                    t = ''.join(s)
                    if ans > t:
                        ans = t
        return ans