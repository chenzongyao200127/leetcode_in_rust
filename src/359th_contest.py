# 356th_contest.py
from typing import List
def isAcronym(words: List[str], s: str) -> bool:
    a = ""
    for w in words:
        a += w[0]
        
    return a == s

# print(isAcronym(words = ["alice","bob","charlie"], s = "abc"))
# print(isAcronym(words = ["an","apple"], s = "a"))
# print(isAcronym(words = ["never","gonna","give","up","on","you"], s = "ngguoy"))


def minSumKAvoiding(n, k):
    nums = []
    num = 1
    while len(nums) < n:
        if k - num not in nums:
            nums.append(num)
        num += 1
    return sum(nums)

# # 示例
# n1, k1 = 5, 4
# print(minSumKAvoiding(n1, k1))  # 输出 18

# n2, k2 = 2, 6
# print(minSumKAvoiding(n2, k2))  # 输出 3



def maxGold(n, offers):
    m = len(offers)
    
    ranges = [0] * m
    for i, (start, end, _) in enumerate(offers):
        for j in range(start, end+1):
            ranges[i] |= (1 << j)

    memo = {}  

    def dp(i, mask):
        if i == m:
            return 0
        if (i, mask) in memo:
            return memo[(i, mask)]

        accept = 0
        if not (ranges[i] & mask):  
            accept = offers[i][2] + dp(i+1, mask | ranges[i])

        reject = dp(i+1, mask)
        memo[(i, mask)] = max(accept, reject)
        return memo[(i, mask)]

    return dp(0, 0)

# # 示例
# n1, offers1 = 5, [[0,0,1],[0,2,2],[1,3,2]]
# print(maxGold(n1, offers1))  # 输出 3

# n2, offers2 = 5, [[0,0,1],[0,2,10],[1,3,2]]
# print(maxGold(n2, offers2))  # 输出 10

def maxGold(n, offers):
    m = len(offers)
    
    # 预处理每个 offer 对应的房屋范围
    ranges = [0] * m
    for i, (start, end, _) in enumerate(offers):
        for j in range(start, end+1):
            ranges[i] |= (1 << j)

    # 找出每个 offer 的冲突
    conflict = [0] * m
    for i in range(m):
        for j in range(m):
            if i != j and (ranges[i] & ranges[j]):
                conflict[i] |= (1 << j)
    
    dp = [0] * (1 << m)
    for mask in range(1 << m):
        valid_offers = [i for i in range(m) if mask & (1 << i)]
        if valid_offers:
            dp[mask] = max(dp[mask], max(dp[mask & (mask ^ conflict[i])] + offers[i][2] for i in valid_offers))

    return dp[(1 << m) - 1]

# # 示例
# n1, offers1 = 5, [[0,0,1],[0,2,2],[1,3,2]]
# print(maxGold(n1, offers1))  # 输出 3

# n2, offers2 = 5, [[0,0,1],[0,2,10],[1,3,2]]
# print(maxGold(n2, offers2))  # 输出 10



# def maxGold(n, offers):
#     m = len(offers)
    
#     # 使用字典保存每个offer的冲突offer
#     conflicts = {i: [] for i in range(m)}
#     for i in range(m):
#         for j in range(m):
#             if i != j and offers[i][1] >= offers[j][0] and offers[i][0] <= offers[j][1]:
#                 conflicts[i].append(j)

#     memo = {}

#     def dfs(index, taken):
#         if index == m:
#             return 0
#         # 先查看此状态是否计算过
#         if (index, tuple(taken)) in memo:
#             return memo[(index, tuple(taken))]
        
#         # 不接受当前offer
#         res = dfs(index+1, taken)
        
#         # 接受当前offer
#         if index not in taken:
#             new_taken = taken.union(set(conflicts[index])).union({index})
#             res = max(res, offers[index][2] + dfs(index+1, new_taken))

#         memo[(index, tuple(taken))] = res
#         return res

#     return dfs(0, set())

# # 示例
# n1, offers1 = 5, [[0,0,1],[0,2,2],[1,3,2]]
# print(maxGold(n1, offers1))  # 输出 3

# n2, offers2 = 5, [[0,0,1],[0,2,10],[1,3,2]]
# print(maxGold(n2, offers2))  # 输出 10

def maxIncome(n, offers):
    # 按金币数量降序排序
    offers.sort(key=lambda x: -x[2])

    # 检查两个offer是否冲突
    def is_conflict(offer1, offer2):
        return not (offer1[1] < offer2[0] or offer1[0] > offer2[1])

    # 递归函数，返回从当前index到offers结尾的最大收入
    def dfs(index, groups):
        if index == len(offers):
            return sum(sum(offer[2] for offer in group) for group in groups)
        
        # 跳过当前offer
        skip = dfs(index+1, groups)

        incomes = [skip]

        # 尝试将当前offer添加到每个现有的分组中
        for group in groups:
            if all(not is_conflict(offers[index], offer) for offer in group):
                group.append(offers[index])
                incomes.append(dfs(index+1, groups))
                group.pop()  # 回溯

        # 创建一个新的分组
        groups.append([offers[index]])
        incomes.append(dfs(index+1, groups))
        groups.pop()  # 回溯

        return max(incomes)

    return dfs(0, [])

# 示例
n = 5
offers = [[0,0,1],[0,2,2],[1,3,2]]
print(maxIncome(n, offers))  # 输出：3

n = 5
offers = [[0,0,1],[0,2,10],[1,3,2]]
print(maxIncome(n, offers))  # 输出：12



class Solution:
    def maximizeTheProfit(self, n: int, offers: List[List[int]]) -> int:
        # Initialize a list of offers ending at each house.
        offers_ending_at = [[] for _ in range(n)]
        
        # Populate the list with offers based on their ending house.
        for start, end, profit in offers:
            offers_ending_at[end].append((start, profit))
        
        # Initialize a list to store maximum profit attainable at each house.
        dp = [0] * n
        
        # Compute the maximum profit for each house.
        for i in range(n):
            # By default, carry over the profit from the previous house.
            if i:
                dp[i] = max(dp[i], dp[i - 1])
            
            # Check for offers ending at the current house and update the profit accordingly.
            for start, profit in offers_ending_at[i]:
                dp[i] = max(dp[i], (dp[start - 1] if start else 0) + profit)
        
        # Return the maximum profit attainable for all houses.
        return dp[-1]


