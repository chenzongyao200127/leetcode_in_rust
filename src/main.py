def minOperations(num1: int, num2: int) -> int:
    MAX = 10**9 + 7
    dp = [MAX] * (num1 + 1)
    dp[num1] = 0
    for i in range(61):
        for j in range(num1, 2**i + num2 - 1, -1):
            if j - 2**i - num2 >= 0:
                dp[j - 2**i - num2] = min(dp[j - 2**i - num2], dp[j] + 1)
    return dp[0] if dp[0] != MAX else -1


print(minOperations(3,-2))



def min_operations(num1: int, num2: int) -> int:
    min_ops = float('inf')
    
    def dfs(num1, num2, ops):
        nonlocal min_ops
        
        if num1 == 0:
            min_ops = min(min_ops, ops)
            return
        
        if ops >= min_ops - 1 or num1 < 0 or abs(num1) > 62 * abs(num2):
            return
        
        for i in range(61):
            dfs(num1 - (2**i + num2), num2, ops + 1)
    
    dfs(num1, num2, 0)
    return min_ops if min_ops != float('inf') else -1


num1 = 3
num2 = -2
print(min_operations(num1, num2))  # 输出：3

num1 = 5
num2 = 7
print(min_operations(num1, num2))  # 输出：-1