# 514_Freedom_Trail
# https://leetcode.cn/problems/freedom-trail/description/

# Input: ring = "godding", key = "gd"
# Output: 4
# Explanation:
# For the first key character 'g', since it is already in place, we just need 1 step to spell this character. 
# For the second key character 'd', we need to rotate the ring "godding" anticlockwise by two steps to make it become "ddinggo".
# Also, we need 1 more step for spelling.
# So the final output is 4.

# Input: ring = "godding", key = "godding"
# Output: 13

# wrong answer
# 这个问题是由于你在旋转ring的时候，旋转到key的每个字符并没有选择最近的位置，而是遍历了ring上的所有位置。
# 这导致在某些情况下，旋转次数超出了预期。
# 现在我将解释这个问题，并给出一个修改后的代码。
# 首先，我们不需要旋转整个字符串。我们只需要知道当前的指针位置，并计算从当前位置到目标字符的最小距离。这样，我们可以避免字符串切片和连接操作，从而提高代码的效率。
# 另外，我们不需要维护一个全局的答案变量，因为我们可以直接从递归函数中返回答案。
# class Solution:
#     def findRotateSteps(self, ring: str, key: str) -> int:
#         n = len(key)
#         ans = float('inf')
#         memo = {}

#         key = ring[0] + key
        
#         def dfs(cur_ring, cnt, times):
#             if (cur_ring, cnt) in memo:
#                 return memo[(cur_ring, cnt)]
            
#             if cnt == len(key):
#                 nonlocal ans
#                 ans = min(ans, times)
#                 return ans
            
#             next_c = key[cnt]
#             local_ans = float('inf')
#             for i in range(len(ring)):
#                 if cur_ring[i] == next_c:
#                     dis = min(i, len(ring) - i)
#                     new_cur_ring = cur_ring[i:] + cur_ring[:i]
#                     local_ans = min(local_ans, dfs(new_cur_ring, cnt+1, times + dis))

#             memo[(cur_ring, cnt)] = local_ans
#             return local_ans

#         dfs(ring, 1, 0)
#         return ans + n

        
class Solution:
    def findRotateSteps(self, ring: str, key: str) -> int:
        memo = {}

        def dfs(index_ring, index_key):
            if index_key == len(key):
                return 0
            
            if (index_ring, index_key) in memo:
                return memo[(index_ring, index_key)]
            
            total_steps = float('inf')
            for i in range(len(ring)):
                if ring[i] == key[index_key]:
                    clockwise = abs(i - index_ring)
                    anticlockwise = len(ring) - clockwise
                    steps = min(clockwise, anticlockwise) + 1 + dfs(i, index_key + 1)
                    total_steps = min(total_steps, steps)

            memo[(index_ring, index_key)] = total_steps
            return total_steps

        return dfs(0, 0)


# 使用动态规划来解决此问题是一个很好的选择。
# 我们可以定义一个二维DP数组`dp[i][j]`，其中`i`表示`key`的字符索引，`j`表示`ring`的字符索引。`dp[i][j]`的值表示，
# 为了匹配`key`的前`i`个字符，当`ring`的第`j`个字符与第`i`个字符匹配时需要的最小步数。

# 状态转移方程如下：


# dp[i][j] = min(dp[i-1][k] + min_steps(k, j)) for all k where ring[k] == key[i-1]


# `min_steps(k, j)`计算从`ring`的`k`位置到`j`位置所需的最小步数。

# 这是使用动态规划方法的代码：

class Solution:
    def findRotateSteps(self, ring: str, key: str) -> int:
        m, n = len(key), len(ring)
        
        # Pre-calculate all positions for every char in the ring
        char_to_indices = {char: [] for char in set(ring)}
        for i, char in enumerate(ring):
            char_to_indices[char].append(i)
        
        # Calculate the minimum steps to move from idx1 to idx2 in ring
        def min_steps(idx1, idx2):
            return min(abs(idx1 - idx2), n - abs(idx1 - idx2))
        
        # Initiate the DP table with large values
        dp = [[float('inf') for _ in range(n)] for _ in range(m)]
        
        # For the first character in key, fill the DP table
        for idx in char_to_indices[key[0]]:
            dp[0][idx] = min_steps(0, idx) + 1
        
        for i in range(1, m):
            for idx in char_to_indices[key[i]]:
                for prev_idx in char_to_indices[key[i-1]]:
                    dp[i][idx] = min(dp[i][idx], dp[i-1][prev_idx] + min_steps(prev_idx, idx) + 1)
        
        return min(dp[m-1])

# Test
sol = Solution()
print(sol.findRotateSteps("ababcab", "acbaacba"))  # Expected output: 17


# 这种方法首先预计算每个字符在`ring`中的所有位置，然后使用动态规划方法来填充`dp`表。


# 【关于无穷大0x3f3f3f的一些知识】

# 0x3f3f3f3f的十进制是1061109567，也就是10^9级别的（和0x7fffffff一个数量级），
# 而一般场合下的数据都是小于10^9的，所以它可以作为无穷大使用而不致出现数据大于无穷大的情形。

# 另一方面，由于一般的数据都不会大于10^9，所以当我们把无穷大加上一个数据时，
# 它并不会溢出（这就满足了“无穷大加一个有穷的数依然是无穷大”），
# 事实上0x3f3f3f3f+0x3f3f3f3f=2122219134，这非常大但却没有超过32-bit int的表示范围，
# 所以0x3f3f3f3f还满足了我们“无穷大加无穷大还是无穷大”的需求。

# 最后，0x3f3f3f3f还能给我们带来一个意想不到的额外好处：
# 如果我们想要将某个数组清零，我们通常会使用memset(a,0,sizeof(a))这样的代码来实现（方便而高效），
# 但是当我们想将某个数组全部赋值为无穷大时（例如解决图论问题时邻接矩阵的初始化），
# 就不能使用memset函数而得自己写循环了（写这些不重要的代码真的很痛苦），
# 我们知道这是因为memset是按字节操作的，它能够对数组清零是因为0的每个字节都是0，
# 现在好了，如果我们将无穷大设为0x3f3f3f3f，那么奇迹就发生了，0x3f3f3f3f的每个字节都是0x3f！
# 所以要把一段内存全部置为无穷大，我们只需要memset(a,0x3f,sizeof(a))。 
#
# ⭐ 所以在通常的场合下，const int INF = 0x3f3f3f3f;真的是一个非常棒的选择。


# 是的，您分享的信息是正确的，并且在算法竞赛和许多编程场景中非常有用。
# 使用`0x3f3f3f3f`作为无穷大的表示在许多算法中是一个常见的技巧，尤其是在需要初始化一个非常大的值时。

# 为了进一步说明，我来列举一些常见的用途：

# 1. **图论算法**：在许多图论算法中，如Floyd-Warshall算法，我们需要初始化一个距离矩阵，
# 使得非邻接顶点之间的距离为无穷大。使用`memset`和`0x3f3f3f3f`可以方便快速地完成这种初始化。

# 2. **动态规划**：在某些动态规划问题中，我们可能需要初始化DP数组为一个非常大的值。同样，这种技巧在这里也很有用。

# 3. **Dijkstra算法**：在Dijkstra的最短路径算法中，我们需要初始化所有顶点到源顶点的距离为无穷大。这种初始化也可以用此技巧来实现。

# 需要注意的是，虽然`0x3f3f3f3f`在大多数情况下都可以作为无穷大使用，但在处理大数据时还是需要格外小心，以确保不会出现溢出或者其他问题。

# 感谢您分享这一有趣且有用的技巧！