# // 167_Two_Sum_II_-_Input_Array_Is_Sorted
# // https://leetcode.cn/problems/two-sum-ii-input-array-is-sorted/

class Solution:
    def twoSum(self, numbers: List[int], target: int) -> List[int]:
        n = len(numbers)
        left = 1
        right = n
        while left < right:
            tmp = numbers[left-1] + numbers[right-1]
            if tmp < target:
                left += 1
            elif tmp > target:
                right -= 1
            else:
                return [left, right]
            
class Solution:
    def twoSum(self, numbers: List[int], target: int) -> List[int]:
        hash_table = {}
        for idx, n in enumerate(numbers):
            res = target - n
            if res not in hash_table:
                hash_table[n] = idx
            else:
                if idx < hash_table[res]:
                    return (idx+1, hash_table[res]+1)
                else:
                    return (hash_table[res]+1, idx+1)


