import math

def xor_sum(nums):
    res = 0
    for num in nums:
        res ^= num
    return res

def beautiful(nums, k):
    for i in range(len(nums)):
        for j in range(i + 1, len(nums)):
            if abs(nums[i] - nums[j]) == k:
                return False
    return True

def count_beautiful_subarrays(nums, k):
    res = 0
    h = {}
    h[0] = 1
    cur = 0

    for num in nums:
        cur ^= num

        if cur in h:
            for sub_end in h[cur]:
                sub_start = sub_end + 1 
                sub = nums[sub_start : sub_start + num]
                if beautiful(sub, k):
                    res += 1
        
        if cur not in h:
            h[cur] = []
        h[cur].append(num)

        return res