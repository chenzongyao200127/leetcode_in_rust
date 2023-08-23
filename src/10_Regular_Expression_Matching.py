# 10_Regular_Expression_Matching
# https://leetcode.cn/problems/regular-expression-matching/description/

# Given an input string s and a pattern p, implement regular expression matching with support for '.' and '*' where:

# '.' Matches any single character.​​​​
# '*' Matches zero or more of the preceding element.
# The matching should cover the entire input string (not partial).

def isMatch(s: str, p: str) -> bool:
    """
    Check if string s matches the pattern p.
    
    :param s: input string
    :param p: pattern string
    :return: boolean indicating if s matches p
    """
    m, n = len(s), len(p)
    
    # dp[i][j] represents if the first i characters of s match the first j characters of p.
    dp = [[False] * (n + 1) for _ in range(m + 1)]
    dp[0][0] = True  # Two empty strings match

    # Initialize for cases when s is an empty string.
    initialize_dp_for_empty_string(dp, p, n)

    for i in range(1, m + 1):
        for j in range(1, n + 1):
            if current_characters_match(s, p, i, j):
                dp[i][j] = dp[i - 1][j - 1]
            elif p[j - 1] == '*':
                handle_asterisk(dp, s, p, i, j)
    
    return dp[m][n]

def initialize_dp_for_empty_string(dp: list, p: str, n: int) -> None:
    """Initialize the dp table for cases when s is an empty string."""
    for j in range(1, n + 1):
        if p[j - 1] == '*':
            dp[0][j] = dp[0][j - 2]

def current_characters_match(s: str, p: str, i: int, j: int) -> bool:
    """Check if current characters of s and p match or if p has a '.'."""
    return s[i - 1] == p[j - 1] or p[j - 1] == '.'

def handle_asterisk(dp: list, s: str, p: str, i: int, j: int) -> None:
    """Handle the '*' character in pattern p."""
    dp[i][j] = dp[i][j - 2] or ((s[i - 1] == p[j - 2] or p[j - 2] == '.') and dp[i - 1][j])

# 示例
s1 = "aa"
p1 = "a"
print(isMatch(s1, p1))  # 输出：false

s2 = "aa"
p2 = "a*"
print(isMatch(s2, p2))  # 输出：true

s3 = "ab"
p3 = ".*"
print(isMatch(s3, p3))  # 输出：true
