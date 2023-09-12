# 1039_多边形三角剖分的最低得分
# https://leetcode.cn/problems/minimum-score-triangulation-of-polygon/description/

# 你有一个凸的 n 边形，其每个顶点都有一个整数值。给定一个整数数组 values ，其中 values[i] 是第 i 个顶点的值（即 顺时针顺序 ）。
# 假设将多边形 剖分 为 n - 2 个三角形。对于每个三角形，该三角形的值是顶点标记的乘积，三角剖分的分数是进行三角剖分后所有 n - 2 个三角形的值之和。
# 返回 多边形进行三角剖分后可以得到的最低分 。

# 示例 1：
# 输入：values = [1,2,3]
# 输出：6
# 解释：多边形已经三角化，唯一三角形的分数为 6。

# 示例 2：
# 输入：values = [3,7,4,5]
# 输出：144
# 解释：有两种三角剖分，可能得分分别为：3*7*5 + 4*5*7 = 245，或 3*4*5 + 3*4*7 = 144。最低分数为 144。

# 示例 3：
# 输入：values = [1,3,1,4,1,5]
# 输出：13
# 解释：最低分数三角剖分的得分情况为 1*1*3 + 1*1*4 + 1*1*5 + 1*1*1 = 13。

# Wrong
# from typing import List

# class Solution:
#     def minScoreTriangulation(self, values: List[int]) -> int:
        
#         def cal_values(values):
#             tmp = 1
#             for n in values:
#                 tmp *= n
#             return tmp
        
#         if len(values) == 3:
#             return cal_values(values)
        
#         def dfs(values, t):
#             if len(values) < 3:
#                 return 0
            
#             if len(values) == 3:
#                 return t + cal_values(values)
            
#             min_val = float("inf")
#             for k in range(1, len(values)-1):
#                 l_values = values[:k+1]
#                 r_values = values[k:]
#                 print(l_values, r_values)
                
#                 cur_val = t + (values[0] * values[k] * values[-1])
#                 new_val = 0
#                 if len(l_values) >= 3:
#                     new_val += dfs(l_values, cur_val)
#                 if len(r_values) >= 3:
#                     new_val += dfs(r_values, cur_val)
                
#                 min_val = min(min_val, new_val)
                
#             return min_val
        
#         return dfs(values, 0) 


# s = Solution()
# ans = s.minScoreTriangulation([1,2,3])
# print(ans)

# s = Solution()
# ans = s.minScoreTriangulation([3,7,4,5])
# print(ans)
    
# s = Solution()
# ans = s.minScoreTriangulation([1,3,1,4,1,5])
# print(ans)




from typing import List
class Solution:
    def minScoreTriangulation(self, values: List[int]) -> int:
        memo = {}
        
        def dfs(i, j):
            if j - i + 1 < 3:
                return 0
            
            if (i, j) in memo:
                return memo[(i, j)]
            
            min_val = float('inf')
            for k in range(i+1, j):
                cur_val = values[i] * values[k] * values[j]
                min_val = min(min_val, dfs(i, k) + cur_val + dfs(k, j))
            
            memo[(i, j)] = min_val
            return min_val
        
        return dfs(0, len(values) - 1)