from typing import List
# def maximumJumps(nums: List[int], target: int) -> int:
#     res = -1
    
#     def backtrack(nums, target, index, jumps):
#         nonlocal res
#         if index == len(nums) - 1:
#             res = max(res, jumps)
#             return
#         for i in range(index + 1, len(nums)):
#             if -target <= nums[i] - nums[index] <= target:
#                 backtrack(nums, target, i, jumps + 1)
    
#     backtrack(nums, target, 0, 0)
#     return -1 if res == -1 else res

from typing import List
def maximumJumps(nums: List[int], target: int) -> int:
    n = len(nums)
    dp = [-1]*n
    dp[0] = 0

    for i in range(1, n):
        for j in range(i):
            if -target <= nums[i] - nums[j] <= target and dp[j] != -1:
                dp[i] = max(dp[i], dp[j] + 1)

    return dp[-1]


# print(maximumJumps(nums = [0,2,1], target = 1))
# print(maximumJumps(nums = [1,3,6,4,1,2], target = 2))
# print(maximumJumps(nums = [1,3,6,4,1,2], target = 3))
# print(maximumJumps(nums = [1,3,6,4,1,2], target = 0))


def maxNonDecreasingLength(nums1: List[int], nums2: List[int]) -> int:
    ans = 0
    
    def maxLong(nums1, nums2):
        n = len(nums1)
    
        prev = min(nums1[0], nums2[0])
        cons = [prev]
        for i in range(1, n):
            if nums1[i] >= prev and nums2[i] >= prev:
                cons.append(min(nums1[i], nums2[i]))
            elif nums1[i] >= prev and nums2[i] < prev:
                cons.append(nums1[i])
            elif nums1[i] < prev and nums2[i] >= prev:
                cons.append(nums2[i])
            else:
                break
            prev = cons[-1]
                
        return len(cons)
    
    for i in range(len(nums1)):
        ans = max(maxLong(nums1[i:].copy(), nums2[i:].copy()), ans)
        if ans >= len(nums1) - i:
            break
    return ans

print(maxNonDecreasingLength([3,19,13,19],[20,18,7,14]))
print(maxNonDecreasingLength([8,7,4],[13,4,4]))
print(maxNonDecreasingLength([1,19,6],[11,6,1]))
print(maxNonDecreasingLength(nums1 = [2,3,1], nums2 = [1,2,1]))
print(maxNonDecreasingLength(nums1 = [1,3,2,1], nums2 = [2,2,3,4]))
print(maxNonDecreasingLength(nums1 = [1,1], nums2 = [2,2]))



def checkArray(nums: List[int], k: int) -> bool:
    n = len(nums)
    diff = [0] * (n + 1)
    curr = 0
    for i in range(n):
        diff[i] += curr
        if i + k <= n:
            diff[i + k] -= curr
        curr += nums[i] - diff[i]
        if curr < 0:
            return False
    return curr == 0


print(checkArray(nums = [2,2,3,1,1,0], k = 3))
print(checkArray(nums = [1,3,1,1], k = 2))
