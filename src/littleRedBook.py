# import sys
# import math


# def read_input():
#     # Reading the first line with two integers
#     n, max_like = map(int, sys.stdin.readline().strip().split())
#     # Reading the relational operators
#     relations = sys.stdin.readline().strip()
#     return n, max_like, relations


# def construct_graph(n, relations):
#     # Construct adjacency matrix
#     g = [[0] * n for _ in range(n)]
#     indegree = [0] * n
#     outdegree = [0] * n

#     for i in range(n - 1):
#         if relations[i] == '<':
#             g[i + 1][i] = 1
#             indegree[i] += 1
#             outdegree[i + 1] += 1
#         elif relations[i] == '>':
#             g[i][i + 1] = 1
#             indegree[i + 1] += 1
#             outdegree[i] += 1

#     return g, indegree, outdegree


# def dfs(node, g, visited, n):
#     # Standard DFS to find the longest path
#     visited.add(node)
#     max_length = 1
#     for i in range(n):
#         if g[node][i] == 1 and i not in visited:
#             max_length = max(max_length, 1 + dfs(i, g, visited, n))
#     visited.remove(node)
#     return max_length


# def count_combinations(n, k):
#     # Calculate nCk
#     if n < k:
#         return 0
#     return math.factorial(n) // (math.factorial(k) * math.factorial(n - k))


# def main():
#     n, max_like, relations = read_input()
#     g, indegree, outdegree = construct_graph(n, relations)

#     md = 10**9 + 7
#     longest_path = 0

#     # Find the longest path in the directed graph
#     for i in range(n):
#         if outdegree[i] > 0:
#             visited = set()
#             path_length = dfs(i, g, visited, n)
#             longest_path = max(longest_path, path_length)

#     # Calculate the number of ways to reach up to max_like using the longest path
#     ans = 0
#     for m in range(longest_path, max_like + 1):
#         ans = (ans + count_combinations(max_like, m)) % md

#     print(ans)


# if __name__ == "__main__":
#     main()

def count_valid_combinations(n, k, relations):
    # DP table where dp[i][v] is the count of valid sequences of length i ending with the value v
    dp = [[0] * (k + 1) for _ in range(n + 1)]

    # Initialize the first row, since any value for the first node is valid
    for v in range(1, k + 1):
        dp[1][v] = 1

    # Fill the DP table using the relations
    for i in range(2, n + 1):
        # We can use prefix sums to efficiently calculate the range sums
        prefix_sum = [0] * (k + 2)  # One extra to handle bounds easily
        for v in range(1, k + 1):
            prefix_sum[v] = prefix_sum[v - 1] + dp[i - 1][v]

        for v in range(1, k + 1):
            if relations[i - 2] == "<":
                # Sum of all dp[i-1][u] where u < v
                dp[i][v] = prefix_sum[v - 1]
            elif relations[i - 2] == "=":
                dp[i][v] = dp[i - 1][v]
            elif relations[i - 2] == ">":
                # Sum of all dp[i-1][u] where u > v
                dp[i][v] = prefix_sum[k] - prefix_sum[v]

    # The total number of valid combinations is the sum of the last row in the DP table
    return sum(dp[n][v] for v in range(1, k + 1))


# Example usage
n = 3
k = 4
relations = ["<", "=", ">"]
# assert (count_valid_combinations(n, k, relations)) == 5


def min_elements(nums, target):
    max_value = target + 1  # 一个比目标大的数用于初始化
    dp = [max_value] * (target + 1)
    dp[0] = 0  # 和为0不需要任何元素

    for num in nums:
        # 从大到小更新dp数组，避免同一个元素多次使用
        for t in range(target, num // 2 - 1, -1):
            if t >= num:
                dp[t] = min(dp[t], dp[t - num] + 1)
            dp[t] = min(dp[t], dp[t - num // 2] + 1)

        # 特殊处理：num 本身不除以2的情形
        for t in range(target, num - 1, -1):
            dp[t] = min(dp[t], dp[t - num] + 1)

    # 如果dp[target]未被更新，则返回-1表示无解
    return dp[target] if dp[target] != max_value else -1


# 示例调用
nums = [1, 2, 3, 4, 6, 10]
target = 8
print(min_elements(nums, target))  # 应返回可能的最小k值 预计是2


def test_min_elements():
    assert min_elements([1, 2, 3, 4, 6, 10], 8) == 2, "Test 1 Failed"
    assert min_elements([10, 20, 40], 15) == 2, "Test 2 Failed"
    assert min_elements([1, 2, 3], 0) == 0, "Test 3 Failed"
    assert min_elements([5, 10, 20], 1) == -1, "Test 4 Failed"
    assert min_elements([9, 10, 11], 5) == 1, "Test 5 Failed"
    assert min_elements([5], 5) == 1, "Test 6 Failed"
    assert min_elements([5], 2) == 1, "Test 7 Failed"
    assert min_elements([1], 1) == 1, "Test 8 Failed"
    assert min_elements([2], 1) == 1, "Test 9 Failed"
    assert min_elements([100, 200, 300], 50) == 1, "Test 10 Failed"
    print("All tests passed!")


test_min_elements()
