# 找到最长的令人敬畏的子串
# https://leetcode.cn/problems/find-longest-awesome-substring/description/

class Solution:
    def longestAwesome(self, s: str) -> int:
        n = len(s)
        prefix = {0: -1}
        ans, sequence = 0, 0

        for j in range(n):
            digit = ord(s[j]) - ord("0")
            sequence ^= (1 << digit)
            if sequence in prefix:
                ans = max(ans, j - prefix[sequence])
            else:
                prefix[sequence] = j

            for k in range(10):
                if sequence ^ (1 << k) in prefix:
                    ans = max(ans, j - prefix[sequence ^ (1 << k)])

        return ans
