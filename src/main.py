
# 这个函数首先初始化了两个指针 left 和 right，以及一个变量 max_length 用于存储最长子数组的长度。变量 even_start 用于记录子数组中第一个偶数元素的位置。

# 在遍历数组的过程中，我们首先检查当前元素是否大于给定的阈值 threshold。如果大于阈值，我们需要更新 left 和 even_start 的值，然后跳过这个元素。

# 接下来，我们检查当前元素是否为偶数。如果是偶数，我们需要更新 even_start 的值（如果 even_start 为 None，则设置为当前位置；如果前一个元素也是偶数，我们需要重新设置 even_start 和 left 的值）。此时，我们可以计算子数组的长度，将其与 max_length 进行比较并更新 max_length 的值。

# 如果当前元素为奇数且前一个元素为偶数，我们需要更新子数组的长度（right - even_start），然后将 left 更新为当前位置。

# 最后，我们将指针 right 向右移动一位，继续遍历数组。当遍历完成后，返回找到的最长子数组的长度。

# 给你一个下标从 0 开始的整数数组 nums 和一个整数 threshold 。
# 请你从 nums 的子数组中找出以下标 l 开头、下标 r 结尾 (0 <= l <= r < nums.length) 且满足以下条件的 最长子数组 ：
# nums[l] % 2 == 0
# 对于范围 [l, r - 1] 内的所有下标 i ，nums[i] % 2 != nums[i + 1] % 2
# 对于范围 [l, r] 内的所有下标 i ，nums[i] <= threshold
# 以整数形式返回满足题目要求的最长子数组的长度。
# 注意：子数组 是数组中的一个连续非空元素序列。

# print(longestAlternatingSubarray([2,3,4,5],4))

def longestAlternatingSubarray(nums, threshold):
    left = 0
    right = 0
    mod = 0
    ans = 0
    while right < len(nums):
        while left < len(nums) and (nums[left] % 2 != 0 or nums[left] > threshold):
            left += 1
            
        print("left", left)
        
        right = left
        mod = 1
        print("right", right)
        while right < len(nums) and nums[right] <= threshold and mod != nums[right] % 2:
            mod = nums[right] % 2
            right += 1
            print("right", right)
            
        ans = max(ans, right - left)
        left = right
            
    
    return ans

print(longestAlternatingSubarray([4,10,3],10))


from collections import deque
from typing import List

# 给你一个下标从 0 开始的整数数组 nums 。nums 的一个子数组如果满足以下条件，那么它是 不间断 的：

# i，i + 1 ，...，j  表示子数组中的下标。对于所有满足 i <= i1, i2 <= j 的下标对，都有 0 <= |nums[i1] - nums[i2]| <= 2 。
# 请你返回 不间断 子数组的总数目。

# 子数组是一个数组中一段连续 非空 的元素序列。

class Solution:
    def continuousSubarrays(self, nums: List[int]) -> int:
        n = len(nums)
        count = 0
        left = 0

        max_queue = deque()
        min_queue = deque()

        for right in range(n):
            while max_queue and nums[right] > nums[max_queue[-1]]:
                max_queue.pop()
            max_queue.append(right)

            while min_queue and nums[right] < nums[min_queue[-1]]:
                min_queue.pop()
            min_queue.append(right)

            while nums[max_queue[0]] - nums[min_queue[0]] > 2:
                left += 1
                if max_queue[0] < left:
                    max_queue.popleft()
                if min_queue[0] < left:
                    min_queue.popleft()

            count += right - left + 1

        return count
    
    
# eratosthenes_sieve 函数实现了厄拉多塞筛法，用于找到所有小于等于 n 的质数。
# prime_pairs_optimized 函数使用了预先计算出的质数集合来查找符合题目要求的质数对。
# 这样，我们避免了在寻找质数对时重复检查每个数是否为质数，从而大大减少了运行时间。
class Solution:
    def findPrimePairs(self, n: int) -> List[List[int]]:
        def eratosthenes_sieve(n):
            is_prime = [True] * (n + 1)
            is_prime[0] = False
            is_prime[1] = False
            prime_list = []

            for i in range(2, int(n**0.5) + 1):
                if is_prime[i]:
                    prime_list.append(i)
                    for multiple in range(i * i, n + 1, i):
                        is_prime[multiple] = False
            for i in range(int(n**0.5) + 1, n + 1):
                if is_prime[i]:
                    prime_list.append(i)

            return prime_list

        primes = set(eratosthenes_sieve(n))
        result = []

        for p in list(primes):
            if n - p in primes and p <= (n // 2) :
                result.append([p, n - p])
        result.sort()
        return result
    
    
class Solution:
    def longestAlternatingSubarray(self, nums: List[int], threshold: int) -> int:
        left = 0
        right = 0
        mod = 0
        ans = 0
        while right < len(nums):
            while left < len(nums) and (nums[left] % 2 != 0 or nums[left] > threshold):
                left += 1


            right = left
            mod = 1
            while right < len(nums) and nums[right] <= threshold and mod != nums[right] % 2:
                mod = nums[right] % 2
                right += 1

            ans = max(ans, right - left)
            left = right


        return ans
