# 371th_contest

# 给你一个下标从 0 开始的整数数组 nums 。如果一对整数 x 和 y 满足以下条件，则称其为 强数对 ：

# |x - y| <= min(x, y)
# 你需要从 nums 中选出两个整数，且满足：这两个整数可以形成一个强数对，并且它们的按位异或（XOR）值是在该数组所有强数对中的 最大值 。

# 返回数组 nums 所有可能的强数对中的 最大 异或值。

# 注意，你可以选择同一个整数两次来形成一个强数对。

from typing import List
from itertools import combinations
class Solution:
    def maximumStrongPairXor(self, nums: List[int]) -> int:
        def is_strong_pair(x, y):
            return abs(x - y) <= min(x, y)
        
        max_xor = 0
        # Consider all pairs (including a number with itself)
        for x, y in combinations(nums + nums, 2):
            if is_strong_pair(x, y):
                max_xor = max(max_xor, x ^ y)
                
        return max_xor


# 给你一个长度为 n 、下标从 0 开始的二维字符串数组 access_times 。对于每个 i（0 <= i <= n - 1 ），access_times[i][0] 表示某位员工的姓名，access_times[i][1] 表示该员工的访问时间。access_times 中的所有条目都发生在同一天内。

# 访问时间用 四位 数字表示， 符合 24 小时制 ，例如 "0800" 或 "2250" 。

# 如果员工在 同一小时内 访问系统 三次或更多 ，则称其为 高访问 员工。

# 时间间隔正好相差一小时的时间 不 被视为同一小时内。例如，"0815" 和 "0915" 不属于同一小时内。

# 一天开始和结束时的访问时间不被计算为同一小时内。例如，"0005" 和 "2350" 不属于同一小时内。

# 以列表形式，按任意顺序，返回所有 高访问 员工的姓名。    

# class Solution:
#     def findHighAccessEmployees(self, access_times: List[List[str]]) -> List[str]:
#         people_set = set()
#         for t in access_times:
#             people_set.add(t[0])
#         peoples = list(people_set)
        
#         tmp = [0 for _ in range(24)]
#         cnts = {}
#         for n in peoples:
#             cnts[n] = tmp.copy()
        
#         print(cnts)
        
#         for t in access_times:
#             name = t[0]
#             time = t[1]
#             hour = int(time[:2])
#             # minu = int(time[2:])
#             cnts[name][hour] += 1
        
#         print(cnts)
        
#         high_frequency_emp = [name for name, hours in cnts.items() if any(visits >= 3 for visits in hours)]
#         return high_frequency_emp

from collections import defaultdict
import bisect

class Solution:
    def findHighAccessEmployees(self, access_times):
        # Create a dictionary to store access times for each employee
        employee_access = defaultdict(list)

        # Process each access time
        for name, time in access_times:
            # Convert time to minutes for easier comparison
            minutes = int(time[:2]) * 60 + int(time[2:])
            employee_access[name].append(minutes)

        high_frequency_emp = []
        for name, times in employee_access.items():
            # Sort the times for each employee
            times.sort()
            for i in range(len(times)):
                # Find the index of the time that is just outside of one hour window
                index = bisect.bisect(times, times[i] + 59, i)
                # If there are 3 or more accesses within one hour, add the employee to the result
                if index - i >= 3:
                    high_frequency_emp.append(name)
                    break

        return high_frequency_emp
        
        
# class Solution:
#     def minOperations(self, nums1: List[int], nums2: List[int]) -> int:    
#         n = len(nums1)
#         memo = {}

#         def conditions_met():
#             return nums1[-1] == max(nums1) and nums2[-1] == max(nums2)

#         def try_swaps(current_swaps, start_index):
#             key = (tuple(nums1), tuple(nums2))
#             if key in memo:
#                 return memo[key]

#             if conditions_met():
#                 memo[key] = current_swaps
#                 return current_swaps

#             if current_swaps == 1001 or start_index >= n:
#                 return float('inf')

#             min_swaps = float('inf')
#             for i in range(start_index, n):
#                 nums1[i], nums2[i] = nums2[i], nums1[i]  # Perform swap
#                 min_swaps = min(min_swaps, try_swaps(current_swaps + 1, i + 1))
#                 nums1[i], nums2[i] = nums2[i], nums1[i]  # Undo swap

#             memo[key] = min_swaps
#             return min_swaps
        
#         if conditions_met():
#             return 0

#         result = try_swaps(0, 0)
#         return result if result != float('inf') else -1
inf = 100000000

class Solution:
    def minOperations(self, nums1: List[int], nums2: List[int]) -> int:
        n = len(nums1)
        
        def getRes(x, y):
            # 复制数组，防止进行意外修改
            a, b = x[:], y[:]
            cnt = 0
            for i in range(n - 1):
                # 发现不合法操作进行修改
                if a[i] > a[-1] or b[i] > b[-1]:
                    cnt += 1
                    a[i], b[i] = b[i], a[i]
            for i in range(n - 1):
                # 所有操作后仍不符合题意，则不存在，返回无穷大
                if a[i] > a[-1] or b[i] > b[-1]:
                    return inf
            return cnt
        
        # 不修改最后一位
        ans = getRes(nums1, nums2)
        
        # 修改最后一位
        nums1[-1], nums2[-1] = nums2[-1], nums1[-1]
        ans = min(ans, getRes(nums1, nums2) + 1)
        
        return ans if ans < inf else -1

# Example usage
sol = Solution()
nums1 = [1, 2, 3, 4]
nums2 = [4, 3, 2, 1]
print(sol.minOperations(nums1, nums2))


nums1 = [1,2,7]
nums2 = [4,5,3]
s = Solution()
ans = s.minOperations(nums1, nums2)
print(ans)


nums1 = [2,3,4,5,9]
nums2 = [8,8,4,4,4]
s = Solution()
ans = s.minOperations(nums1, nums2)
print(ans)

nums1 = [1,5,4]
nums2 = [2,5,3]
s = Solution()
ans = s.minOperations(nums1, nums2)
print(ans)


nums1 = [14,28,68,68,65,67,68]
nums2 = [68,68,49,64,68,67,67]
s = Solution()
ans = s.minOperations(nums1, nums2)
print(ans)
# 输出：
# -1
# 预期：
# 3

nums1 = [1,1,10]
nums2 = [1,5,1]
s = Solution()
ans = s.minOperations(nums1, nums2)
print(ans)



# class Solution:
#     def maximumStrongPairXor(self, nums):
#         nums.sort()
#         max_xor = 0

#         for i in range(len(nums)):
#             # Use binary search to find the farthest element that forms a strong pair with nums[i]
#             lo, hi = i + 1, len(nums)
#             while lo < hi:
#                 mid = lo + (hi - lo) // 2
#                 if abs(nums[i] - nums[mid]) <= min(nums[i], nums[mid]):
#                     lo = mid + 1
#                 else:
#                     hi = mid
            
#             # Compute XOR for all pairs in range [i+1, lo)
#             for j in range(i + 1, lo):
#                 max_xor = max(max_xor, nums[i] ^ nums[j])

#         return max_xor

# class TrieNode:
#     def __init__(self):
#         self.children = {}
#         self.numbers = []

# class Solution:
#     def maximumStrongPairXor(self, nums):
#         # Build Trie
#         root = TrieNode()
#         for num in nums:
#             node = root
#             for i in range(31, -1, -1):
#                 bit = (num >> i) & 1
#                 if bit not in node.children:
#                     node.children[bit] = TrieNode()
#                 node = node.children[bit]
#                 node.numbers.append(num)

#         max_xor = 0
#         for num in nums:
#             node = root
#             candidate = 0
#             for i in range(31, -1, -1):
#                 bit = (num >> i) & 1
#                 opposite_bit = bit ^ 1
#                 if opposite_bit in node.children and any(abs(num - other) <= min(num, other) for other in node.children[opposite_bit].numbers):
#                     candidate |= (opposite_bit << i)
#                     node = node.children[opposite_bit]
#                 else:
#                     candidate |= (bit << i)
#                     node = node.children[bit]

#             max_xor = max(max_xor, num ^ candidate)

#         return max_xor

# # Example usage
# sol = Solution()
# nums = [1, 2, 3, 4]
# ans = sol.maximumStrongPairXor(nums)
# print(ans)

# s = Solution()
# ans = s.maximumStrongPairXor([500,520,2500,3000])
# print(ans)