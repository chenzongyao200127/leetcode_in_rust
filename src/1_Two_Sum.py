# 1. Two Sum
# https://leetcode.cn/problems/two-sum/

class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        n = len(nums)
        for i in range(n):
            for j in range(i + 1, n):
                if nums[i] + nums[j] == target:
                    return [i, j]
        
        return []
    
class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        hashtable = dict()
        for i, num in enumerate(nums):
            if target - num in hashtable:
                return [hashtable[target - num], i]
            hashtable[nums[i]] = i
        return []
    
# class Solution {
# public:
#     vector<int> twoSum(vector<int>& nums, int target) {
#         unordered_map<int, int> hashtable;
#         for (int i = 0; i < nums.size(); ++i) {
#             auto it = hashtable.find(target - nums[i]);
#             if (it != hashtable.end()) {
#                 return {it->second, i};
#             }
#             hashtable[nums[i]] = i;
#         }
#         return {};
#     }
# };


# class Solution {
#     public int[] twoSum(int[] nums, int target) {
#         Map<Integer, Integer> hashtable = new HashMap<Integer, Integer>();
#         for (int i = 0; i < nums.length; ++i) {
#             if (hashtable.containsKey(target - nums[i])) {
#                 return new int[]{hashtable.get(target - nums[i]), i};
#             }
#             hashtable.put(nums[i], i);
#         }
#         return new int[0];
#     }
# }


# func twoSum(nums []int, target int) []int {
#     hashTable := map[int]int{}
#     for i, x := range nums {
#         if p, ok := hashTable[target-x]; ok {
#             return []int{p, i}
#         }
#         hashTable[x] = i
#     }
#     return nil
# }
