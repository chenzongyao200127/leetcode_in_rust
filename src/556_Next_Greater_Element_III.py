# 556_Next_Greater_Element_III
# https://leetcode.cn/problems/next-greater-element-iii/


class Solution:
    def nextGreaterElement(self, n: int) -> int:
        # 31_Next_Permutation
        def nextPermutation(nums) -> None:
            def find_decreasing_suffix_start(nums) -> int:
                for i in range(len(nums) - 1, 0, -1):
                    if nums[i - 1] < nums[i]:
                        return i
                return 0

            def find_swap_index(nums, start) -> int:
                for i in range(len(nums) - 1, start - 1, -1):
                    if nums[i] > nums[start - 1]:
                        return i

            suffix_start = find_decreasing_suffix_start(nums)

            if suffix_start == 0:
                nums = [-1]
                return

            swap_index = find_swap_index(nums, suffix_start)
            nums[suffix_start - 1], nums[swap_index] = nums[swap_index], nums[suffix_start - 1]
            nums[suffix_start:] = sorted(nums[suffix_start:]) 


        n = str(n)
        nums = []
        for i in range(len(n)):
            nums.append(n[i:i+1])   
               
        nextPermutation(nums)
        nums = nums[::-1]
        ans = 0
        for i in range(len(nums)):
            ans += (int(nums[i]) - int('0')) * 10**i
        if str(ans) == n:
            return -1
        else:
            return ans if ans <= 2 ** 31 - 1 else -1
        
        
        
class Solution:
    def nextGreaterElement(self, n: int) -> int:
        nums = list(str(n))
        n = len(nums)

        if n <= 1:
            return -1

        i, j, k = n - 2, n - 1, n - 1

        # find: A[i] < A[j]
        while i >= 0 and nums[i] >= nums[j]:
            i -= 1
            j -= 1

        # 不存在符合条件的最小整数
        if i < 0:
            return -1

        # find: A[i]<A[k]
        while nums[k] <= nums[i]:
            k -= 1
        #  swap A[i], A[k]
        nums[i], nums[k] = nums[k], nums[i]

        # reverse A[j:end]
        nums[j:] = nums[j:][::-1]

        n = int(''.join(nums))

        return n if n <= 2 ** 31 - 1 else -1
