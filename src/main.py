# 给你一个下标从 0 开始的整数数组 nums ，它包含 n 个 互不相同 的正整数。如果 nums 的一个排列满足以下条件，我们称它是一个特别的排列：

# 对于 0 <= i < n - 1 的下标 i ，要么 nums[i] % nums[i+1] == 0 ，要么 nums[i+1] % nums[i] == 0 。
# 请你返回特别排列的总数目，由于答案可能很大，请将它对 109 + 7 取余 后返回。

def num_permutations(nums):
    MOD = 10**9 + 7
    n = len(nums)
    
    # 使用字典来存储已经计算过的子问题的结果
    memo = {}

    def backtrack(index, path):
        nonlocal memo
        if index == n:
            return 1

        # 如果子问题已经计算过，直接返回结果
        if tuple(path) in memo:
            return memo[tuple(path)]

        count = 0
        for i in range(n):
            if nums[i] not in path and (index == 0 or path[-1] % nums[i] == 0 or nums[i] % path[-1] == 0):
                path.append(nums[i])
                count += backtrack(index + 1, path)
                count %= MOD
                path.pop()

        # 将子问题的结果存储到字典中
        memo[tuple(path)] = count
        return count

    return backtrack(0, [])

# 示例 1
result = num_permutations([2, 3, 6])
print(result)  # 输出：2

# 示例 2
result = num_permutations([1, 4, 3])
print(result)  # 输出：2

result = num_permutations([31, 93])
print(result)  # 输出：2


def min_cost(cost, time):
    n = len(cost)
    total_time = sum(time)
    
    dp = [[float('inf')] * (total_time + 1) for _ in range(n + 1)]
    dp[0][0] = 0

    for i in range(1, n + 1):
        for j in range(1, total_time + 1):
            if j >= time[i - 1]:
                dp[i][j] = min(dp[i][j], dp[i - 1][j - time[i - 1]] + cost[i - 1])
            if j >= 1:
                dp[i][j] = min(dp[i][j], dp[i - 1][j - 1])
    
    return min(dp[n])