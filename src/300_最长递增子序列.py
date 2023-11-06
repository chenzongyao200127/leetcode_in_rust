# 300_最长递增子序列
# https://leetcode.cn/problems/longest-increasing-subsequence/description/



from typing import List

class Solution:
    def lengthOfLIS(self, nums: List[int]) -> int:
        cnts = [1] * len(nums)
        
        def findMaxCountForNextLargerNum(cur, zip_nums):
            return max((count for num, count in zip_nums if num > cur), default=0)
        
        for i in range(len(nums)-2, -1, -1):
            cur = nums[i]
            zip_nums = list(zip(nums[i+1:], cnts[i+1:]))
            cnts[i] = 1 + findMaxCountForNextLargerNum(cur, zip_nums)
        
        return max(cnts)

s = Solution()
ans = s.lengthOfLIS([10,9,2,5,3,7,101,18])
print(ans)  # Expected output: 4

# 思路错误
# from typing import List

# from pprint import pprint

# class Solution:
#     def lengthOfLIS(self, nums: List[int]) -> int:
#         cnts = [1] * len(nums)
        
#         def findNextNum(cur, zip_nums):
#             l = 0
#             r = len(zip_nums)-1
#             zip_nums = sorted(zip_nums)
#             # pprint(zip_nums)
#             while l < r:
#                 mid = l + (r - l) // 2
#                 # print(mid)
#                 if zip_nums[mid][0] > cur:
#                     r = mid
#                 else:
#                     l = mid + 1
#             return zip_nums[r][1]
        
#         for i in range(len(nums)-2, -1, -1):
#             cur = nums[i]
#             tmp_nums = nums[i+1:]
#             tmp_cnts = cnts[i+1:]
#             zip_nums = list(zip(tmp_nums, tmp_cnts))
#             cnts[i] = findNextNum(cur, zip_nums) + 1
        
#         return max(cnts)

# # s = Solution()
# # ans = s.lengthOfLIS([10,9,2,5,3,7,101,18])
# # print(ans)
        
# s = Solution()
# ans = s.lengthOfLIS([7,7,7,7,7,7,7])
# print(ans)        


from typing import List

class Solution:
    def lengthOfLIS(self, nums: List[int]) -> int:
        tails = []
        
        for num in nums:
            # Perform binary search to find the position to replace/insert
            l, r = 0, len(tails) - 1
            while l <= r:
                mid = l + (r - l) // 2
                if tails[mid] < num:
                    l = mid + 1
                else:
                    r = mid - 1
            
            if l == len(tails):
                tails.append(num)
            else:
                tails[l] = num
        
        return len(tails)

s = Solution()
ans = s.lengthOfLIS([10,9,2,5,3,7,101,18])
print(ans)  # Expected output: 4
