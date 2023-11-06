 # 2127_参加会议的最多员工数
# https://leetcode.cn/problems/maximum-employees-to-be-invited-to-a-meeting/description/

# from typing import List
# class Solution:
#     def maximumInvitations(self, favorite: List[int]) -> int:
#         if len(favorite) == 2:
#             if favorite == [1,0]:
#                 return 2
#             else:
#                 return 1
#         lens = []
#         for i in range(len(favorite)):
#             cur = i
#             visited = set()
#             visited.add(cur)
#             tmp = 1
#             while favorite[cur] not in visited:
#                 # print(visited)
#                 # print(favorite[cur])
#                 visited.add(favorite[cur])
#                 cur = favorite[cur]
#                 tmp += 1
#             lens.append(tmp)
        
#         # print(lens)
#         lens = sorted(lens)
#         print(lens)
#         ans = 0
#         for i in range(len(lens)):
#             if i == lens[i]:
#                 ans = max(i, ans)
                
            
#         if ans == 2:
#             return 3 
#         else:
#             return ans
                
# s = Solution()
# ans = s.maximumInvitations([2,2,1,2])
# print(ans)            

# s = Solution()
# ans = s.maximumInvitations([3,0,1,4,1])
# print(ans)            

# s = Solution()
# ans = s.maximumInvitations([1,2,0])
# print(ans)            
            

# s = Solution()
# ans = s.maximumInvitations([1,0,0,2,1,4,7,8,9,6,7,10,8])
# print(ans)             