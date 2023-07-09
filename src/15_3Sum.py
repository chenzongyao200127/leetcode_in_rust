# 15_3Sum
# https://leetcode.cn/problems/3sum/

# 给你一个整数数组 nums ，判断是否存在三元组 [nums[i], nums[j], nums[k]] 满足 i != j、i != k 且 j != k ，
# 同时还满足 nums[i] + nums[j] + nums[k] == 0 。请
# 你返回所有和为 0 且不重复的三元组。
# 注意：答案中不可以包含重复的三元组。

# 示例 1：
# 输入：nums = [-1,0,1,2,-1,-4]
# 输出：[[-1,-1,2],[-1,0,1]]
# 解释：
# nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0 。
# nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0 。
# nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0 。
# 不同的三元组是 [-1,0,1] 和 [-1,-1,2] 。
# 注意，输出的顺序和三元组的顺序并不重要。

# 示例 2：
# 输入：nums = [0,1,1]
# 输出：[]
# 解释：唯一可能的三元组和不为 0 。

# 示例 3：
# 输入：nums = [0,0,0]
# 输出：[[0,0,0]]
# 解释：唯一可能的三元组和为 0 。

from typing import List

# So, we essentially need to find three numbers x, y, and z such that they add up to the given value. 
# If we fix one of the numbers say x, we are left with the two-sum problem at hand!

# For the two-sum problem, if we fix one of the numbers, say x, 
# we have to scan the entire array to find the next number y, which is value - x where value is the input parameter. 
# Can we change our array somehow so that this search becomes faster?

# The second train of thought for two-sum is, without changing the array, 
# can we use additional space somehow? Like maybe a hash map to speed up the search?

# 首先对数组进行排序，然后遍历排序后的数组。
# 对于每个元素，使用两个指针分别指向它后面的最小和最大元素，然后调整两个指针的位置来找到三个元素的和为0的情况。
# 为了避免重复的三元组，我们在移动指针的过程中检查当前元素是否和上一个元素相等。

# 经典面试题
def threeSum(nums):
    nums.sort()
    res = []
    for i in range(len(nums)-2):
        if i > 0 and nums[i] == nums[i-1]:
            continue
        l, r = i+1, len(nums)-1
        while l < r:
            s = nums[i] + nums[l] + nums[r]
            if s < 0:
                l += 1
            elif s > 0:
                r -= 1
            else:
                res.append((nums[i], nums[l], nums[r]))
                # 继续遍历
                while l < r and nums[l] == nums[l+1]:
                    l += 1
                while l < r and nums[r] == nums[r-1]:
                    r -= 1
                l += 1; r -= 1
    return res



# 我们可以使用一个哈希表来解决这个问题。具体的算法是这样的：

# 首先对输入的数组进行排序。
# 然后，对于每个元素，我们尝试找到数组中的两个元素，它们的和等于当前元素的相反数。
# 为了快速找到这样的两个元素，我们可以在遍历数组的过程中，使用一个哈希表来存储每个元素和它的索引。
# 这样，对于每个元素，我们可以在 O(1) 的时间内查找哈希表，看看是否存在两个元素的和等于当前元素的相反数。
# 注意，在查找过程中，我们需要确保不会使用同一个元素两次。
def threeSum(nums):
    res = []
    nums.sort()
    for i in range(len(nums) - 2):
        if i > 0 and nums[i] == nums[i - 1]:
            continue
        
        d = {}
        for j in range(i + 1, len(nums)):
            print(d)
            if nums[j] in d:
                if d[nums[j]] == 1:
                    res.append([nums[i], -nums[i] - nums[j], nums[j]])
                    d[nums[j]] -= 1
            else:
                d[-nums[i] - nums[j]] = 1
    return res

print(threeSum([-1,0,1,2,-1,-4]))
# 这个代码首先对输入的数组进行了排序，然后遍历了数组中的每一个元素，对于每个元素，它使用了一个哈希表来存储已经遍历过的元素和它们出现的次数。、
# 在遍历过程中，如果发现当前的元素在哈希表中存在，那么就找到了一个三元组，将其添加到结果中，同时减少哈希表中对应元素的数量。
# 如果当前元素在哈希表中不存在，那么就在哈希表中添加一个新的元素，这个元素的值是当前元素和前面元素之和的相反数，它的数量是1。



# 官解
class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        n = len(nums)
        nums.sort()
        ans = list()
        
        # 枚举 a
        for first in range(n):
            # 需要和上一次枚举的数不相同
            if first > 0 and nums[first] == nums[first - 1]:
                continue
            
            # c 对应的指针初始指向数组的最右端
            third = n - 1
            target = -nums[first]
            
            # 枚举 b
            for second in range(first + 1, n):
                # 需要和上一次枚举的数不相同
                if second > first + 1 and nums[second] == nums[second - 1]:
                    continue
                # 需要保证 b 的指针在 c 的指针的左侧
                while second < third and nums[second] + nums[third] > target:
                    third -= 1
                # 如果指针重合，随着 b 后续的增加
                # 就不会有满足 a+b+c=0 并且 b<c 的 c 了，可以退出循环
                if second == third:
                    break
                
                if nums[second] + nums[third] == target:
                    ans.append([nums[first], nums[second], nums[third]])
        
        return ans

# 官方题解有两个地方可以优化：
# 设 s = nums[first] + nums[first+1] + nums[first+2]，如果 s > 0，由于数组已经排序，后面无论怎么选，
# 选出的三个数的和不会比 s 还小，所以只要 s > 0 就可以直接 break 外层循环了。

# 如果 nums[first] + nums[n-2] + nums[n-1] < 0，由于数组已经排序，nums[first] 加上后面任意两个数都是小于 0 的，
# 所以下面的双指针就不需要跑了。但是后面可能有更大的 nums[first]，所以还需要继续枚举，continue 外层循环。