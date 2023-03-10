// 1590_Make_Sum_Divisible_by_P
// https://leetcode.cn/problems/make-sum-divisible-by-p/


/// 前缀和+同余+哈希表
use std::collections::HashMap;
impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let mut total = 0;
        for num in nums.iter() {
            total = (total + (num.rem_euclid(p))).rem_euclid(p);
        }           
        if total == 0 {
            return 0;
        }
        let mut map: HashMap<i32, i32> = HashMap::new(); //用哈希表记录前缀和对应的最大下标
        map.insert(0, -1); // 这里千万别漏了
        let mut pre_sum = 0;
        let mut ans = nums.len() as i32;
        for i in 0..nums.len() {
            pre_sum = (pre_sum + (nums[i].rem_euclid(p))).rem_euclid(p);
            let target = (pre_sum - total.rem_euclid(p)).rem_euclid(p);
            if let Some(j) = map.get(&target) {
                ans = ans.min((i - *j as usize) as i32);
            }
            map.insert(pre_sum, i as i32);
        }

        if ans == nums.len() as i32 {
            -1
        } else {
            ans
        }
    }
}

/// 示例代码
use std::collections::HashMap;
impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let n = nums.len();
        
        let mut prefix = vec![0;n+1];
        
        nums.iter().enumerate().for_each(|(i,&num)|
            prefix[i+1] = (prefix[i]+num).rem_euclid(p)
        );
        let rem = prefix[n];
        if rem == 0 {return 0;}
        
        let mut index_map = HashMap::new();
        let mut res = n;
        
        for i in 0..n+1 {
            let s = prefix[i];
            let t = (s+rem).rem_euclid(p);
            if index_map.contains_key(&s) {
                res = res.min(i-index_map[&s]);
            }
            index_map.insert(t,i);
        }
        
        if res < n { res as _ } else {-1}
    }
}


// class Solution {
//     public:
//         int minSubarray(vector<int>& nums, int p) {
//             int target=0,ans=nums.size(),sum=0,res;
//             for(int &num:nums)//计算目标
//                 target=(target+num)%p;
//             if(target==0)//整除，直接返回
//                 return 0;
//             unordered_map<int,int> pos;//用哈希表记录前缀和对应的最大下标
//             pos[0]=-1;
//             for(int i=0;i<nums.size();++i)
//             {
//                 sum=(sum+nums[i])%p,res=(sum-target+p)%p;//计算前缀和，以及要查找的和
//                 pos[sum]=i;//记录这个前缀和
//                 ans=min(ans,pos.find(res)==pos.end()?INT_MAX:i-pos[res]);//查找并比较
//             }
//             return ans==nums.size()?-1:ans;
//         }
//     };


















// class Solution:
//     def minSubarray(self, nums: List[int], p: int) -> int:
//         x = sum(nums) % p
//         if x == 0:
//             return 0
//         y = 0
//         index = {0: -1}
//         ans = len(nums)
//         for i, v in enumerate(nums):
//             y = (y + v) % p
//             if (y - x) % p in index:
//                 ans = min(ans, i - index[(y - x) % p])
//             index[y] = i
//         return ans if ans < len(nums) else -1